# Safety model

Everything here acts on a live game, where a mistake costs player data or money rather than a redeploy. Three rules hold everywhere:

**Writes are dry runs by default.** A write describes what it would do and stops. `--apply` performs it, after a confirmation. Forgetting a flag is the safe outcome.

**`--env all` is refused.** Each env is a different experience; nothing acts on production because it matched a glob.

**Scopes are the real boundary.** A key that cannot write cannot be made to write by anything here. See [API keys and scopes](scopes.md).

## Confirmations

| Command | Before writing |
| --- | --- |
| `ban add` / `remove`, `group rank` / `unrank`, `restart launch`, `data` writes (not `snapshot` or `restore-store`), `ads` writes | Asks to confirm; `--yes` skips |
| `data delete-store` | Asks you to type the store's name |
| `data` overwrites | Also saves the current value to `.rbx/backups/` first |
| `memorystore set` / `delete`, `message`, `secret set` / `delete` | No question: `--apply` is the gate (cache values, messages and secrets are written by scripts) |

`--apply --yes` is an unattended write: right in a reviewed scheduled job, wrong as a habit in a terminal.

## `--json` and prompts

A command that would ask cannot also own stdout. So writes that ask take `--json` only together with `--yes`, and the rest do not take `--json` at all. Each command page lists which.
