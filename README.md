# rbx-ops

Operate a live Roblox experience from the command line: its servers, players, data, the messages and secrets its servers read, and its ad campaigns.

**Documentation: <https://rbx-forge.github.io/rbx-ops/>**

```sh
rbx-ops servers list --env prod --status crashed
rbx-ops ban add builderman --reason "exploit" --duration 7d --env prod          # dry run
rbx-ops ban add builderman --reason "exploit" --duration 7d --env prod --apply  # asks, then sends
```

## Install

With [mise](https://mise.jdx.dev) or [Rokit](https://github.com/rojo-rbx/rokit), either of which pins the latest release in your project's config:

```bash
mise use --pin github:rbx-forge/rbx-ops
rokit add rbx-forge/rbx-ops   # or, with Rokit
```

Or take a binary for your platform from the [releases page](https://github.com/rbx-forge/rbx-ops/releases), which ships a `SHA256SUMS` alongside the archives, or build from source:

```bash
cargo install --git https://github.com/rbx-forge/rbx-ops rbx-ops --locked
```

## Commands

| Command | What it does |
| --- | --- |
| [`servers`](./docs/servers.md) | Live and terminated servers, and a crashed server's logs |
| [`analytics`](./docs/analytics.md) | Players, retention, revenue, funnels. CSV for charting |
| [`ban`](./docs/ban.md) | Inspect and change player restrictions |
| [`group`](./docs/group.md) | A group's roles and members, and moving members between roles |
| [`restart`](./docs/restart.md) | Forecast and launch a rolling server restart |
| [`data`](./docs/data.md) | Read, overwrite, copy and recover data store entries; leaderboards |
| [`memorystore`](./docs/memorystore.md) | Write values servers read through `MemoryStoreService` |
| [`message`](./docs/message.md) | Send a MessagingService message to every server |
| [`secret`](./docs/secret.md) | Secrets `HttpService:GetSecret` reads, sent encrypted |
| [`ads`](./docs/ads.md) | Launch and steer ad campaigns. Spends money |
| [`user`](./docs/user.md) | Username to id and back. Hidden from `--help` |

Every write is a dry run until `--apply`, and `--env all` is refused: see the [safety model](./docs/reference/safety.md). Each command needs its own Open Cloud scope: see [API keys and scopes](./docs/reference/scopes.md).

## Testing

The `servers` and `analytics` clients are tested against recorded production responses, in `crates/rbx-servers/tests/fixtures/` and `crates/rbx-analytics/tests/fixtures/`. They are byte for byte what Roblox sent, except `playerIds`, `jobId`, pagination tokens and analytics operation paths, which are replaced with synthetic values; every figure in `docs/` is invented. Recordings rather than hand-written JSON, because the specification is wrong or silent about several fields:

- `nextPageToken` is `""` on `cloud/v2` and `null` on `server-management`. Reading `""` as a token requests the same page forever.
- `uptime` is a .NET `TimeSpan`, `[d.]hh:mm:ss[.fffffff]`, not an ISO 8601 duration.
- `frameRate` is `null` on a new server and `0` on a stopped one.
- `dataPoints` needs camelCase renaming; without it every analytics query returns an empty series.

## License

[MPL-2.0](LICENSE). A community tool, not affiliated with Roblox Corporation.
