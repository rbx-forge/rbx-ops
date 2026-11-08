# rbx-ops ban

Inspect and change who is allowed into the experience.

```sh
rbx-ops ban status builderman --env prod
rbx-ops ban add builderman --reason "exploit: fly hack" --display-reason "Banned 7 days for cheating" --duration 7d --env prod
rbx-ops ban remove builderman --env prod --apply
```

## Naming a player

Every subcommand takes any of these, mixed:

```text
156                                        a user id (bare digits are always an id)
builderman                                 a username
name:12345                                 a username made of digits
https://www.roblox.com/users/156/profile   a profile link
```

`@12345` also forces a username, but in PowerShell `@` must be quoted; `name:` works everywhere. A username that does not exist is an error. Display names are not usernames.

## rbx-ops ban status

```text
builderman (156)
  https://www.roblox.com/users/156/profile
  RESTRICTED  for 7d
  since        2026-08-01T10:12:03Z
  your note    exploit: fly hack, clip of 3 Aug
  player sees  Banned 7 days for cheating
```

#### `--json`

```json
{
  "schema_version": 1,
  "env": "prod",
  "universe_id": "5544332211",
  "count": 1,
  "players": [
    {
      "user_id": "156",
      "username": "builderman",
      "display_name": "builderman",
      "profile_url": "https://www.roblox.com/users/156/profile",
      "restricted": true,
      "permanent": false,
      "duration": "604800s",
      "start_time": "2026-08-01T10:12:03Z",
      "private_reason": "exploit: fly hack, clip of 3 Aug",
      "display_reason": "Banned 7 days for cheating"
    }
  ]
}
```

`permanent`, `duration`, `start_time` and the reasons are absent for a player who is not restricted.

## rbx-ops ban list

Everyone currently restricted, by id (the endpoint returns no names).

| Flag | Meaning |
| --- | --- |
| `--limit <n>` | Maximum rows |
| `--include-inactive` | Include lifted restrictions |
| `--json` | One JSON document |

```json
{
  "schema_version": 1,
  "env": "prod",
  "universe_id": "5544332211",
  "include_inactive": false,
  "limit": 100,
  "limit_reached": false,
  "count": 2,
  "restrictions": [
    { "user_id": "156", "active": true, "permanent": false, "duration": "604800s", "private_reason": "exploit: fly hack" },
    { "user_id": "881", "active": true, "permanent": true }
  ]
}
```

!!! warning "Permanent has no duration"
    Roblox expresses a permanent ban by omitting the duration. Read `permanent`, never `.duration // "none"`.

## rbx-ops ban logs

The audit trail of restriction changes. `--limit <n>` caps it.

## rbx-ops ban add

```sh
rbx-ops ban add builderman \
  --reason "exploit: fly hack, clip of 3 Aug" \
  --display-reason "Banned 7 days for cheating" \
  --duration 7d \
  --env prod
```

Without `--apply`, nothing is sent: you see the resolved account with its profile link, and the exact payload. With `--apply`, you are asked to confirm. Check the profile first: there is more than one Builderman.

| Flag | Meaning |
| --- | --- |
| `--reason` | Required. Private note, 1000 characters |
| `--display-reason` | Shown to the player, 400 characters |
| `--duration` | `30m`, `12h`, `7d`, `2w`. **Omit for permanent** |
| `--allow-alts` | Do not extend the ban to the player's alt accounts (Roblox extends it by default) |
| `--apply` | Send it |
| `--yes` | Skip the confirmation |

`--duration permanent` is refused: permanent is reached only by leaving the flag out.

## rbx-ops ban remove

Lift a restriction. Takes `--apply` and `--yes` like `add`.

## Scopes

| Subcommand | Scope |
| --- | --- |
| `status`, `list`, `logs` | `universe.user-restriction:read` |
| `add`, `remove` | `universe.user-restriction:write` |

Writes carry an idempotency key, so a retried request is not applied twice.
