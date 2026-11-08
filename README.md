# rbx-ops

Operate a live Roblox experience from the command line: what its servers are doing, how the ones that stopped ended, what its analytics say, who is allowed in, what players have stored, what reaches every running server, and the ad campaigns that bring players in.

| Command | What it does | Docs |
| --- | --- | --- |
| `servers` | Live and terminated servers, and the logs of one that crashed. | [docs/servers.md](./docs/servers.md) |
| `analytics` | Your own metrics: players, retention, ARPPU. CSV for charting elsewhere. | [docs/analytics.md](./docs/analytics.md) |
| `ban` | Inspect and change player restrictions. | [docs/ban.md](./docs/ban.md) |
| `group` | List a group's roles and move members between them. | [docs/group.md](./docs/group.md) |
| `restart` | Forecast and launch a rolling server restart. | [docs/restart.md](./docs/restart.md) |
| `data` | Read, overwrite, copy and recover a data store entry; `data ordered` for leaderboards. | [docs/data.md](./docs/data.md) |
| `memorystore` | Write cache values servers read through `MemoryStoreService`. | [docs/memorystore.md](./docs/memorystore.md) |
| `message` | Push a MessagingService message to every running server. | [docs/message.md](./docs/message.md) |
| `secret` | The secrets `HttpService:GetSecret` reads, written encrypted. | [docs/secret.md](./docs/secret.md) |
| `ads` | Launch and steer ad campaigns. Spends money, reads no results. | [docs/ads.md](./docs/ads.md) |
| `user` | A username to an id, or an id to a username. Hidden from `--help`. | [docs/user.md](./docs/user.md) |

## Install

```bash
rokit add rbx-forge/rbx-ops
```

Or take a binary for your platform from the [releases page](https://github.com/rbx-forge/rbx-ops/releases), which ships a `SHA256SUMS` alongside the archives, or build from source:

```bash
cargo install --git https://github.com/rbx-forge/rbx-ops rbx-ops --locked
```

## Environments: `rbxplace.toml`

`--env <name>` names an experience in the project's `rbxplace.toml`; `--universe-id` and `--place-id` name one directly, with no file at all.

```toml
[prod]
universe_id = 9876543210
places.main = 123456789012345
confirm = true   # destructive writes on this env ask first

[staging]
universe_id = 9876543211
places.main = 234567890123456
```

```sh
rbx-ops servers list --env prod
rbx-ops ban status builderman --universe-id 9876543210
```

## What kind of tool this is

Everything here acts on state that only exists at runtime, and none of it comes from a file you commit.

| | Acts on | Source of truth | A mistake costs |
| --- | --- | --- | --- |
| this tool | the live game | the live game | player data, irreversibly |

Banning a player has no desired state in a TOML file: it is a consequence of what happened in your game last night. So nothing here reconciles a repository against Roblox, and nothing here belongs in a pipeline that runs on every push.

## Getting a key

These commands authenticate with an Open Cloud API key, from `--api-key` or `RBX_API_KEY`. Create it in the Creator Hub, bound to the experiences it acts on, with only the scopes the commands you run need:

| Command | Scope | Read or write |
| --- | --- | --- |
| `servers` | `universe:read` | read |
| `analytics` | `universe.analytics:read` | read |
| `ban status` / `list` / `logs` | `universe.user-restriction:read` | read |
| `ban add` / `remove` | `universe.user-restriction:write` | **write** |
| `group roles` / `members` | `group:read` | read |
| `group rank` / `unrank` | `group:read` + `group:write` | **write** |
| `restart forecast` / `status` | `universe:read` | read |
| `restart launch` | `universe:write` | **write** |
| `data` reads | `universe-datastores.objects:read,list` | read |
| `data` writes | `universe-datastores.objects:create,update` + `universe-datastores.control:create` | **write** |
| `data revisions` / `restore` | `universe-datastores.versions:list,read` | read |
| `data ordered` reads | `universe.ordered-data-store.scope.entry:read` | read |
| `data ordered` writes | `universe.ordered-data-store.scope.entry:write` | **write** |
| `memorystore get` / `list` | `memory-store.sorted-map:read` | read |
| `memorystore set` / `delete` | `memory-store.sorted-map:write` | **write** |
| `message` | `universe-messaging-service:publish` | **write** |
| `secret list` / `public-key` | `universe.secret:read` | read |
| `secret set` | `universe.secret:read` + `universe.secret:write` | **write** |
| `secret delete` | `universe.secret:write` | **write** |
| `ads` | `ad.campaign:read`, `ad.campaign:write`; `ad.billing:read` for `accounts` | **write** |

**Keep read and write in separate keys.** Roblox binds a key to its scopes and universes when you create it, so a read-only key is read-only no matter what calls it. The read key is the one that ends up in a shell history during a debugging session, and it should be the one that cannot ban anybody.

## Safety model

Three rules, all structural rather than conventions to remember.

**Writes are dry-run by default.** Any operation that changes something describes what it would do and stops. `--apply` performs it, and prompts. The safe outcome is what happens when you forget a flag.

**`--env all` is refused.** Each env is a different experience, and a command that quietly acted on production because it matched a glob is a command nobody can trust.

**Scopes are the real boundary.** A key that cannot write cannot be made to write by anything on this side.

## The Studio cookie

A `.ROBLOSECURITY` cookie is a complete account identity: not scoped to a universe or an operation, not revocable per tool, strictly more powerful than any key. Only `group members`, when it is given a role, can use one, to list that role's holders through the endpoint the Creator Hub uses instead of reading every member. It is taken from `--cookie` or `RBX_COOKIE`, or from a signed-in Studio **if you say yes**: an interactive run asks once, a run with nowhere to ask refuses, `--auto-cookie` and `--no-auto-cookie` are the standing answers. Everything else takes an API key and nothing else.

## Testing and fixtures

Every client here is tested against **recorded production responses**, in `crates/rbx-*/tests/fixtures/`. Everything is byte for byte what Roblox sent, except four things replaced with synthetic values: `playerIds`, `jobId`, pagination tokens and analytics operation paths.

The last two are not obvious. A pagination token is an opaque base64 blob, but decoding one shows Roblox packs real data inside it, including a `LastGameId` that is a live server's job id. The tests only ever ask whether a token exists, never what is in it, so an opaque placeholder costs nothing.

**Place versions are kept as recorded.** A version number is a publish counter, and the universe and place ids in these files are placeholders, so it is attached to nothing. Replacing them would cost the property the recordings exist for: the ordering and the numeric-versus-string sorting of real version numbers is exactly the kind of detail a hand-written fixture gets wrong. Documentation is the opposite case: every figure in `docs/` is invented.

Recordings rather than hand-written JSON, because the specification is wrong or silent about several fields, and a hand-written fixture would encode the specification and agree with the bug:

- `nextPageToken` is `""` on `cloud/v2` and `null` on `server-management`. Reading `""` as a token requests the same page forever.
- `uptime` is a .NET `TimeSpan`, `[d.]hh:mm:ss[.fffffff]`, not an ISO 8601 duration. All three forms occur.
- `frameRate` is `null` on a new server and `0` on a stopped one.
- `dataPoints` needs camelCase renaming; without it every analytics query returns an empty series and reports "no data".

## License

[MPL-2.0](LICENSE)
