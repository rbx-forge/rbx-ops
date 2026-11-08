# API keys and scopes

Create keys in the [Creator Hub](https://create.roblox.com/dashboard/credentials), bound to the experiences they act on, with only the scopes below.

!!! tip "Keep read and write in separate keys"
    A key's scopes are fixed when you create it, so a read-only key stays read-only whatever calls it. The read key is the one that ends up in a terminal while debugging; it should be the one that cannot ban anybody.

| Command | Scope | |
| --- | --- | --- |
| `servers` | `universe:read` | read |
| `analytics` | `universe.analytics:read` | read |
| `ban status` / `list` / `logs` | `universe.user-restriction:read` | read |
| `ban add` / `remove` | `universe.user-restriction:write` | **write** |
| `group roles` / `member` / `members` | `group:read` | read |
| `group rank` / `unrank` | `group:read` + `group:write` | **write** |
| `restart forecast` / `status` | `universe:read` | read |
| `restart launch` | `universe:write` | **write** |
| `data` reads | `universe-datastores.objects:read,list` | read |
| `data` writes | `universe-datastores.objects:create,update` + `universe-datastores.control:create` | **write** |
| `data delete-key` | `universe-datastores.objects:delete` | **write** |
| `data stores` | `universe-datastores.control:list` | read |
| `data delete-store` / `restore-store` | `universe-datastores.control:delete` | **write** |
| `data snapshot` | `universe-datastores.control:snapshot` | **write** |
| `data revisions` / `restore-key` | `universe-datastores.versions:list,read` | read |
| `data ordered` reads | `universe.ordered-data-store.scope.entry:read` | read |
| `data ordered` writes | `universe.ordered-data-store.scope.entry:write` | **write** |
| `memorystore get` / `list` | `memory-store.sorted-map:read` | read |
| `memorystore set` / `delete` | `memory-store.sorted-map:write` | **write** |
| `message` | `universe-messaging-service:publish` | **write** |
| `secret list` / `public-key` | `universe.secret:read` | read |
| `secret set` | `universe.secret:read` + `universe.secret:write` | **write** |
| `secret delete` | `universe.secret:write` | **write** |
| `ads` | `ad.campaign:read`, `ad.campaign:write`; `ad.billing:read` for `accounts` | **write** |
| `user` | none | read |

A `403` naming a missing scope is reported with that scope. The usual cause is a different key loaded in `RBX_API_KEY`, not a key lacking the scope.
