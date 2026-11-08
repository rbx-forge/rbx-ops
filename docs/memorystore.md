# rbx-ops memorystore

Read and write memory store sorted map items: values a script outside Roblox sets, and every server reads through `MemoryStoreService:GetSortedMap()`. Needs `memory-store.sorted-map:read` to read and `:write` to write.

```sh
rbx-ops memorystore --map Cache set rotation --value '{"map":"desert"}' --ttl 300s --apply --env prod
rbx-ops memorystore --map Cache get rotation --env prod
```

`--map` names the map, which exists from its first write. Queues and `flush` are not supported.

## rbx-ops memorystore set

`rbx-ops memorystore set <item>`

Create or update an item (one request either way). Without `--apply`, describes the write. No confirmation: these are cache values with a TTL, written by scripts.

| Flag | Meaning |
| --- | --- |
| `--value <json>` / `--file <path>` | The value |
| `--ttl <d>` | `300s`, `10m`, `2h`. Without it the item stays until deleted |
| `--sort-key <n>` / `--string-sort-key <s>` | Ordering for `list` |
| `--apply` | Write |

## rbx-ops memorystore get

`rbx-ops memorystore get <item>`

The value as JSON on stdout, the expiry on stderr. A missing item is an error. `--out <path>` writes to a file.

`--json`:

```json
{ "schema_version": 1, "map": "Cache", "item": "rotation", "expire_time": "2026-08-15T09:43:49Z", "value": { "map": "desert" } }
```

`expire_time` is absent without a TTL.

## rbx-ops memorystore list

| Flag | Meaning |
| --- | --- |
| `--limit <n>` | Maximum items |
| `--values` | Include each value |
| `--json` | `{ map, limit, limit_reached, count, items: [{ id, numeric_sort_key \| string_sort_key, expire_time, value }] }` |

An empty map and a map whose items all expired look the same.

## rbx-ops memorystore delete

`rbx-ops memorystore delete <item> --apply`

## Telling servers

Servers see a new value when their own code next reads the map. To make them read now, send a [message](message.md) with a reference to the item (messages are capped at 1 KB, items are not):

```sh
rbx-ops memorystore --map Cache set rotation --file rotation.json --ttl 1h --apply --env prod
rbx-ops message --topic cache --payload '{"key":"rotation"}' --apply --env prod
```
