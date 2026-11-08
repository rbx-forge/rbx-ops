# rbx-ops user

A username to a user id, or the reverse. Public: no key, no file. Hidden from `--help`.

```sh
rbx-ops user builderman
rbx-ops user 156 builderman name:12345
rbx-ops user builderman --id
```

Arguments take what [`ban`](ban.md#naming-a-player) takes. An unknown name or id is an error. Display names are not usernames.

```text
builderman (156)
somebody "Some Body" (1234567890)
```

| Flag | Prints |
| --- | --- |
| `--id` | The bare id, one per line |
| `--name` | The bare username, one per line |
| `--json` | An array of `{ user_id, username, display_name, has_verified_badge, profile_url }` |

`--id` composes with keys that embed an id:

```sh
rbx-ops data --datastore PlayerData get "User_$(rbx-ops user builderman --id)" --env prod
```
