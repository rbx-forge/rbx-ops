# Configuration

## Targeting an experience

`--env <name>` names an experience in the project's `rbxplace.toml`. `--universe-id` and `--place-id` name one directly, with no file at all.

```toml
[owner]            # read by `group` when --group is not passed
type = "group"
id = 12345678

[prod]
universe_id = 9876543210
places.main = 123456789012345

[staging]
universe_id = 9876543211
places.main = 234567890123456
```

```sh
rbx-ops servers list --env prod
rbx-ops ban status builderman --universe-id 9876543210
```

`--env all` and group names are refused: each env is a different live experience. Only `servers` needs a place, and takes it from the env (`--place` picks one) or from `--place-id`.

## Global flags

Accepted before or after the command name.

| Flag | Env var | Description |
| --- | --- | --- |
| `--api-key <key>`, `-a` | `RBX_API_KEY` | Open Cloud API key. Prefer the variable: a flag ends up in shell history |
| `--env <name>`, `-e` | | An env from `rbxplace.toml` |
| `--universe-id <id>` | | A universe, without the file. Wins over `--env` |
| `--place-id <id>` | | A place, without the file |
| `--place <name>` | | A place within the env |
| `--places <path>` | | Path to `rbxplace.toml` |
| `--cookie <value>` | `RBX_COOKIE` | Session cookie (see below) |
| `--auto-cookie` / `--no-auto-cookie` | | Use a signed-in Studio without asking / never |

## The Studio cookie

A `.ROBLOSECURITY` cookie is a whole account session, more powerful than any key. Only `group members`, given a role, uses one, to list that role's holders directly. It comes from `--cookie` or `RBX_COOKIE`, or from a signed-in Studio **if you say yes**: an interactive run asks once, a run with nobody to ask refuses, and `--auto-cookie` / `--no-auto-cookie` are the standing answers. Everything else uses the API key only.
