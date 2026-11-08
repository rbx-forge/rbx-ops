# rbx-ops data

Read, overwrite, copy and recover data store entries, from a script or across envs. For browsing and visual diffs, a GUI such as [DataStoria](https://github.com/this-fifo/DataStoria) is better.

```sh
rbx-ops data --datastore PlayerData get Player_156 --env prod
rbx-ops data --datastore PlayerData reset Player_156 --env prod --apply
rbx-ops data --datastore PlayerData copy Player_156 --from prod --to staging --apply
```

`--datastore` names the store and `--scope` its scope (default `global`; a wrong scope reads as "not found"). Writes are dry runs without `--apply`, and ask before writing.

!!! warning "An overwrite cannot be undone through the API"
    After `set` or `reset`, the previous value is gone at once: Roblox keeps no revision of it, unless a [snapshot](#rbx-ops-data-snapshot) was taken before. So every write first saves the current value to `.rbx/backups/<env>/<entry>-<UTC time>.json`. Add `.rbx/` to `.gitignore`: it is player data.

## Entries

### rbx-ops data get

`rbx-ops data get <entry>`

| Flag | Meaning |
| --- | --- |
| `--out <path>` | Write the value to a file |
| `--json` | One JSON document |

### rbx-ops data set

`rbx-ops data set <entry>`

```sh
rbx-ops data --datastore PlayerData set Player_156 --value '{"coins":0}' --env prod --apply
rbx-ops data --datastore PlayerData set Player_156 --file profile.json --env prod --apply
```

| Flag | Meaning |
| --- | --- |
| `--value <json>` / `--file <path>` | The new value |
| `--drop-metadata` | Also drop the entry's `users` and `attributes` (kept by default: `users` is how Roblox answers a player's data request) |
| backup flags | See [Backups](#backups) |
| `--apply`, `--yes`, `--json` | As above; `--json` needs `--yes` |

### rbx-ops data reset

`rbx-ops data reset <entry>`

Overwrite with a fresh profile: `--template <path>`, or `playerdata.template.json` in the working directory. There is no built-in profile. The dry run shows the current value beside the new one. Takes the backup flags, `--apply`, `--yes`, `--json`.

### rbx-ops data delete-key

`rbx-ops data delete-key <entry>`

`RemoveAsync`, from outside the game. Gentler than a reset: the game sees no entry and builds its own fresh profile, and the old value stays readable through `revisions` for thirty days. Takes the backup flags, `--apply`, `--yes`, `--json`. Needs `universe-datastores.objects:delete`.

!!! warning "Delete while the player is offline"
    A live session still holding the profile writes it back when it ends, undoing the delete or reset.

### rbx-ops data increment

`rbx-ops data increment <entry> --by <n>`

Atomic add: two concurrent grants both land, unlike read-then-set. `--apply`, `--yes`.

### rbx-ops data copy

`rbx-ops data copy <entry> --from <env> --to <env>`

Copy an entry between envs, to reproduce a bug on real data. `--to-entry <key>` writes under another key. Both envs are explicit, and the destination keeps its own `users`. Takes the backup flags, `--apply`, `--yes`.

### rbx-ops data list

Keys of the store.

| Flag | Meaning |
| --- | --- |
| `--prefix <p>` | Keys starting with this |
| `--show-deleted` | Include deleted entries |
| `--limit <n>` | Maximum keys |
| `--json` | `{ datastore, scope, prefix, show_deleted, limit, limit_reached, count, entries: [{ id }] }` |

### rbx-ops data revisions

`rbx-ops data revisions <entry>`

The revisions Roblox kept: after a delete, or after a snapshot. Expect fewer than you wrote. `--revision <id>` prints that revision's value. `--json`.

### rbx-ops data restore-key

`rbx-ops data restore-key <entry> --revision <id>`

Write a revision back. Takes the backup flags, `--apply`, `--yes`, `--json`. Without a revision to restore, put a backup back with `set --file .rbx/backups/<env>/<file>.json`.

### rbx-ops data diff

`rbx-ops data diff <entry>`

```sh
rbx-ops data --datastore PlayerData diff Player_156 --revisions <a>,<b> --env prod --open
rbx-ops data --datastore PlayerData diff Player_156 --between prod,staging --open
```

Both sides are written to files. `--open` hands them to `$RBX_DIFF_TOOL`, else `code --diff`, else `git diff --no-index`; otherwise the paths are printed. `--json` gives the two paths, and cannot be combined with `--open`.

## Backups

Before every overwrite, the current value goes to `.rbx/backups/<env>/<entry>-<UTC time>.json`, next to `rbxplace.toml`.

| Flag | Meaning |
| --- | --- |
| `--keep <n>` | Keep the last `n` copies of this entry (default 10) |
| `--backup <path>` | Write the copy exactly there instead |
| `--no-backup` | No copy. Only after a snapshot, or where nothing can be written |

## Stores

### rbx-ops data stores

Every store of the experience. A store exists from its first write.

| Flag | Meaning |
| --- | --- |
| `--show-deleted` | Include deleted stores |
| `--limit <n>` | Maximum stores |
| `--json` | `{ show_deleted, limit, count, limit_reached, stores: [{ id, create_time, deleted }] }` |

### rbx-ops data delete-store

`rbx-ops data delete-store <store>`

The store is named positionally; `--datastore` is ignored. With `--apply`, you type the store's name back to confirm (`--yes` skips). Deleted stores are listed by `stores --show-deleted` and come back with `restore-store`; Roblox does not say for how long. `--json` needs `--yes`.

### rbx-ops data restore-store

`rbx-ops data restore-store <store>`

Undo a `delete-store`. `--apply`, `--json`. Both need `universe-datastores.control:delete`, which also grants the undo.

### rbx-ops data snapshot

After a snapshot, the next write to **every** key keeps the replaced value as a revision for 30 days. One per experience per UTC day (a second call reports the first). It protects values as they were when taken, so take it right before the risky write. `--apply`. Needs `universe-datastores.control:snapshot`.

## Ordered data stores

Leaderboards: integer values, sorted by Roblox, no revisions and no backups.

```sh
rbx-ops data ordered list --datastore Highscores --env prod
rbx-ops data ordered list --datastore Highscores --limit 100 --asc --min 1000 --max 5000
rbx-ops data ordered get Player_156 --datastore Highscores
rbx-ops data ordered set Player_156 4200 --datastore Highscores
rbx-ops data ordered increment Player_156 -50 --datastore Highscores
rbx-ops data ordered delete Player_156 --datastore Highscores
```

| Verb | Does |
| --- | --- |
| `list` | Descending by default. `--limit`, `--asc`, `--min`, `--max` (applied by Roblox), `--json` |
| `get <entry>` | One value. `--json` |
| `set <entry> <value>` | Exact value, creating it unless `--no-create` |
| `increment <entry> <n>` | Atomic add; negative subtracts. Prefer it whenever two writers touch a key |
| `delete <entry>` | Remove |

The writes ask first, showing the current value; `--yes` skips. Scopes: `universe.ordered-data-store.scope.entry:read` to read, `:write` to write.

## `--json`

| Takes `--json` | Subcommands |
| --- | --- |
| Always | `get`, `list`, `revisions`, `diff`, `stores`, `restore-store`, `ordered list`, `ordered get` |
| With `--yes` only | `set`, `reset`, `restore-key`, `delete-key`, `delete-store` |
| Never | `copy`, `increment`, `snapshot`, the `ordered` writes |

The stored value is nested as JSON, not escaped into a string, so `jq .value.coins` works:

```json
{
  "schema_version": 1,
  "datastore": "PlayerData",
  "scope": "global",
  "entry": "Player_156",
  "found": true,
  "deleted": false,
  "revision_id": "08DEF1A1B5E3ADA9.0000000002.01",
  "value": { "coins": 500, "level": 12 }
}
```

`found` is `false` for a missing key (exit `0`). `value` is absent with `--out` (then `out` holds the path). `users` and `attributes` are never included.

Writes report:

```json
{
  "schema_version": 1,
  "datastore": "PlayerData",
  "scope": "global",
  "entry": "Player_156",
  "action": "set",
  "applied": true,
  "existed": true,
  "revision_id": "08DF077D....01",
  "backup": ".rbx/backups/prod/Player_156-20260831T163538Z.json"
}
```

`action` is `set`, `reset`, `restore` or `delete`; `applied` is `false` on a dry run; `revision_id` is what `revisions --revision` takes. Store writes report `{ datastore, action: "delete-store" | "restore-store", applied }`.

## Scopes

| Use | Scope |
| --- | --- |
| Read entries | `universe-datastores.objects:read,list` |
| Write entries | `universe-datastores.objects:create,update` and `universe-datastores.control:create` |
| `delete-key` | `universe-datastores.objects:delete` |
| `revisions`, `restore-key` | `universe-datastores.versions:list,read` |
| `stores` | `universe-datastores.control:list` |
| `delete-store`, `restore-store` | `universe-datastores.control:delete` |
| `snapshot` | `universe-datastores.control:snapshot` |
| Ordered stores | `universe.ordered-data-store.scope.entry:read` / `:write` |
