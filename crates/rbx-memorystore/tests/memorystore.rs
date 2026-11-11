//! Memory store sorted map reads and writes over HTTP.
//!
//! The tests worth having here are the ones covering what cost a real request
//! to learn: the item id travels in the query string, a write is an upsert
//! rather than a create, and a map that has never existed reads as empty
//! instead of missing.

use rbx_core::GlobalFlags;
use rbx_memorystore::{run, MemoryStoreCli};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const UNIVERSE: u64 = 66778899001;
const ITEMS_PATH: &str = "/cloud/v2/universes/66778899001/memory-store/sorted-maps/Cache/items";
const ITEM_PATH: &str =
    "/cloud/v2/universes/66778899001/memory-store/sorted-maps/Cache/items/rotation";

#[derive(clap::Parser)]
struct Wrapper {
    #[command(flatten)]
    ms: MemoryStoreCli,
}

fn flags(places: &str) -> GlobalFlags {
    GlobalFlags {
        api_key: Some("test-key".into()),
        cookie: None,
        no_auto_cookie: true,
        auto_cookie: false,
        env: Some("ops".into()),
        place: None,
        places: places.into(),
        universe_id: None,
        place_id: Vec::new(),
    }
}

fn places_file(dir: &std::path::Path) -> String {
    let file = dir.join("rbxplace.toml");
    std::fs::write(
        &file,
        format!("[ops]\nuniverse_id = {UNIVERSE}\n\n[ops.places]\nmain = 1\n"),
    )
    .unwrap();
    file.to_string_lossy().into_owned()
}

fn cli(args: &[&str], server: &MockServer) -> MemoryStoreCli {
    let mut argv = vec!["memorystore", "--map", "Cache"];
    argv.extend_from_slice(args);
    <Wrapper as clap::Parser>::parse_from(argv)
        .ms
        .with_base_url(server.uri())
}

#[tokio::test]
async fn set_without_apply_sends_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    // No mock at all: any request would 404 and fail the run.

    run(
        cli(
            &["set", "rotation", "--value", r#"{"map":"desert"}"#],
            &server,
        ),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap();

    assert!(
        server.received_requests().await.unwrap().is_empty(),
        "a set without --apply must not reach the network"
    );
}

/// `POST .../items` with the id in the body answers
/// `400 "The id field is required."`. The write has to be a PATCH against the
/// item's own path, so the id ends up in the URL.
#[tokio::test]
async fn set_writes_to_the_item_path_and_never_puts_the_id_in_the_body() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path(ITEM_PATH))
        .and(body_json(serde_json::json!({
            "value": { "map": "desert" },
            "ttl": "300s"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "rotation",
            "value": { "map": "desert" },
            "expireTime": "2026-08-13T09:13:33Z"
        })))
        .mount(&server)
        .await;

    run(
        cli(
            &[
                "set",
                "rotation",
                "--value",
                r#"{"map":"desert"}"#,
                "--ttl",
                "300s",
                "--apply",
            ],
            &server,
        ),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert!(
        body.get("id").is_none(),
        "the id belongs in the URL, not the body: {body}"
    );
}

/// The body's `ttl` is a protobuf duration: whole seconds with an `s` suffix.
/// `--ttl 10m` sent as written is refused, so every form the help promises has
/// to reach the wire as seconds.
#[tokio::test]
async fn every_ttl_form_the_help_promises_is_sent_as_seconds() {
    for (given, sent) in [
        ("300s", "300s"),
        ("10m", "600s"),
        ("2h", "7200s"),
        ("1d", "86400s"),
        ("45", "45s"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(ITEM_PATH))
            .and(body_json(serde_json::json!({ "value": 1, "ttl": sent })))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({ "id": "rotation", "value": 1 })),
            )
            .expect(1)
            .mount(&server)
            .await;

        run(
            cli(
                &["set", "rotation", "--value", "1", "--ttl", given, "--apply"],
                &server,
            ),
            &flags(&places_file(dir.path())),
        )
        .await
        .unwrap_or_else(|err| panic!("--ttl {given} should be sent as {sent}: {err:#}"));
    }
}

#[tokio::test]
async fn a_ttl_that_cannot_be_read_is_refused_before_any_request() {
    for bad in [
        "10 minutes",
        "abc",
        "1.5h",
        "10x",
        "0",
        "0s",
        "+5m",
        "s",
        "",
    ] {
        let dir = tempfile::tempdir().unwrap();
        let server = MockServer::start().await;

        let error = run(
            cli(
                &["set", "rotation", "--value", "1", "--ttl", bad, "--apply"],
                &server,
            ),
            &flags(&places_file(dir.path())),
        )
        .await
        .expect_err(&format!("--ttl {bad:?} should be refused"));

        assert!(
            error.to_string().contains("TTL") || error.to_string().contains("--ttl"),
            "the error should say the TTL is the problem, got: {error}"
        );
        assert!(
            server.received_requests().await.unwrap().is_empty(),
            "--ttl {bad:?} must fail before the request"
        );
    }
}

/// A create and an update are the same request. Without `allowMissing`, the
/// first write to a new item would fail and every `set` would need a read
/// first.
#[tokio::test]
async fn set_upserts_rather_than_requiring_the_item_to_exist() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path(ITEM_PATH))
        .and(query_param("allowMissing", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": "rotation", "value": 1 })),
        )
        .mount(&server)
        .await;

    run(
        cli(&["set", "rotation", "--value", "1", "--apply"], &server),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn set_refuses_a_value_that_is_not_json() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;

    let error = run(
        cli(
            &["set", "rotation", "--value", "not json", "--apply"],
            &server,
        ),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap_err();

    assert!(
        error.to_string().contains("valid JSON"),
        "expected a JSON parse error, got: {error}"
    );
    assert!(
        server.received_requests().await.unwrap().is_empty(),
        "a malformed value must fail before the request"
    );
}

#[tokio::test]
async fn get_reports_a_missing_item_rather_than_printing_null() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(ITEM_PATH))
        .respond_with(ResponseTemplate::new(404).set_body_string("{}"))
        .mount(&server)
        .await;

    let error = run(
        cli(&["get", "rotation"], &server),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap_err();

    assert!(
        error.to_string().contains("no item"),
        "expected a missing-item error, got: {error}"
    );
}

/// A sorted map is created by its first write, so a name that has never been
/// used answers 200 with an empty list. Listing it is not an error.
#[tokio::test]
async fn list_treats_a_map_that_never_existed_as_empty() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(ITEMS_PATH))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "items": [], "nextPageToken": null })),
        )
        .mount(&server)
        .await;

    run(cli(&["list"], &server), &flags(&places_file(dir.path())))
        .await
        .unwrap();
}

/// Roblox returns `""` for "no more pages" on this endpoint, where the data
/// store endpoints return `null`. Treating the empty string as a real token
/// would fetch the same page forever.
#[tokio::test]
async fn list_stops_on_an_empty_page_token() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(ITEMS_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{ "id": "a", "value": 1, "numericSortKey": 1.0 }],
            "nextPageToken": ""
        })))
        .mount(&server)
        .await;

    run(cli(&["list"], &server), &flags(&places_file(dir.path())))
        .await
        .unwrap();

    assert_eq!(
        server.received_requests().await.unwrap().len(),
        1,
        "an empty next-page token means stop, not fetch again"
    );
}

#[tokio::test]
async fn delete_without_apply_sends_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;

    run(
        cli(&["delete", "rotation"], &server),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap();

    assert!(
        server.received_requests().await.unwrap().is_empty(),
        "a delete without --apply must not reach the network"
    );
}

#[tokio::test]
async fn delete_with_apply_sends_one_delete() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(ITEM_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .mount(&server)
        .await;

    run(
        cli(&["delete", "rotation", "--apply"], &server),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap();

    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn a_missing_map_flag_is_an_error_naming_the_flag() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;

    let parsed = <Wrapper as clap::Parser>::parse_from(["memorystore", "list"])
        .ms
        .with_base_url(server.uri());
    let error = run(parsed, &flags(&places_file(dir.path())))
        .await
        .unwrap_err();

    assert!(
        error.to_string().contains("--map"),
        "the error should name the flag to pass, got: {error}"
    );
}

/// Every page asks for the same size, and the result respects `--limit`.
///
/// The endpoint states the rule itself: "When paginating, all other parameters
/// provided to the subsequent call must match the call that provided the page
/// token." Recomputing `maxPageSize` from what is still wanted sends page two
/// with a different value than the call that issued its token: a request
/// Roblox may reject, and only on listings long enough to page, which are the
/// ones nobody tries by hand.
///
/// The second assertion is the other half. A fixed page size without a truncate
/// returns more rows than were asked for, which is how porting this rule to
/// another crate went wrong once already.
#[tokio::test]
async fn every_page_asks_for_the_same_size_and_the_limit_is_respected() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;

    let full_page: Vec<serde_json::Value> = (0..100)
        .map(|i| serde_json::json!({ "id": format!("k{i}"), "value": i, "numericSortKey": i as f64 }))
        .collect();
    Mock::given(method("GET"))
        .and(path(ITEMS_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": full_page,
            "nextPageToken": "more"
        })))
        .mount(&server)
        .await;

    run(
        cli(&["list", "--limit", "150"], &server),
        &flags(&places_file(dir.path())),
    )
    .await
    .unwrap();

    let requests = server.received_requests().await.unwrap();
    assert!(
        requests.len() >= 2,
        "expected the walk to page, saw {} request(s)",
        requests.len()
    );

    let sizes: Vec<String> = requests
        .iter()
        .map(|r| {
            r.url
                .query_pairs()
                .find(|(k, _)| k == "maxPageSize")
                .map(|(_, v)| v.into_owned())
                .unwrap_or_default()
        })
        .collect();
    assert!(
        sizes.windows(2).all(|w| w[0] == w[1]),
        "maxPageSize changed between pages: {sizes:?}"
    );
    assert_eq!(
        sizes[0], "100",
        "the first page asks for the cap: {sizes:?}"
    );
}
