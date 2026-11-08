# rbx-ops secret

Store the credentials the game uses and the repository must never hold. The server gets them as a `Secret` that Luau can send but not read:

```lua
HttpService:RequestAsync({
    Url = "https://api.example.com/v1/orders",
    Method = "POST",
    Headers = { Authorization = HttpService:GetSecret("api_token") },
})
```

```sh
printenv API_TOKEN | rbx-ops secret set api_token --stdin --domain api.example.com --apply --env prod
rbx-ops secret list --env prod
```

Values are sealed against the universe's public key (a LibSodium sealed box) before leaving your machine: the ciphertext cannot be opened by anything but the universe, so it is safe in a CI log.

!!! warning "Nothing can read a secret back"
    Roblox never returns a stored value, and a replaced value is gone for good. Keep the original in your password manager.

## rbx-ops secret set

`rbx-ops secret set <name>`

Create or replace. Without `--apply`, describes the write.

| Flag | Meaning |
| --- | --- |
| `--stdin` | Read the value from stdin, one trailing newline stripped. Preferred |
| `--file <path>` | Read a file, byte for byte |
| `--value <text>` | On the command line: visible in the process list and in command-line audit logs |
| `--domain <pattern>` | Hosts it may be sent to: `api.example.com`, `*.example.com`, `*` |
| `--no-domain` | Never sent; for signing inside the server |
| `--apply` | Write |
| `--json` | A receipt (only with `--apply`) |

`--domain` or `--no-domain` is required on every write, rotations included, because a write replaces the domain too. `--no-domain` on an API token means it is never attached, and the far end answers `401`.

Names: letters, digits and `_`, 1 to 64, not starting with a digit (`api-key` is invalid).

!!! warning "Windows PowerShell 5.1 corrupts piped values"
    Its pipes turn non-ASCII bytes into `?`, and the secret is stored that way. Use `pwsh` 7. Compare the `N bytes` that `set` reports with the length you expect: it is the only check available.

`--json`:

```json
{ "schema_version": 1, "universe_id": 7654321, "id": "api_token", "action": "updated", "bytes": 38, "domain": "api.example.com", "key_id": "key-2026-08" }
```

`action` is `created` or `updated` (a previous value is gone). `bytes` is the plaintext length, enough to catch an empty pipe.

Rotating is the same command. Servers see the new value on their next `GetSecret`; to make them read it now, [send a message](message.md) naming the secret, never the value. Rotate each env with its own value, in a loop: there is no `--env all`.

## rbx-ops secret list

Names, domains, key ids and dates; never values.

```text
api_token  api.example.com  updated 2026-08-02T10:00:00Z
signing    (no domain: server-side use only)  updated 2026-07-11T14:22:03Z
```

| Flag | Meaning |
| --- | --- |
| `--limit <n>` | Maximum secrets |
| `--json` | `{ universe_id, limit, limit_reached, count, secrets: [{ id, domain, key_id, create_time, update_time }] }`; `domain` absent when none |

## rbx-ops secret delete

`rbx-ops secret delete <name> --apply`

Immediate and irreversible: a server calling `GetSecret` on it starts failing. `--json` with `--apply`.

## rbx-ops secret public-key

The universe's X25519 public key (stdout) and its `key_id` (stderr), for sealing elsewhere. `--json`. Secrets sealed under an older key:

```sh
current=$(rbx-ops secret public-key --env prod --json | jq -r .key_id)
rbx-ops secret list --env prod --json | jq -r --arg k "$current" '.secrets[] | select(.key_id != $k) | .id'
```

## Scopes and limits

| Operation | Scope |
| --- | --- |
| `list`, `public-key` | `universe.secret:read` |
| `set` | `universe.secret:read` and `universe.secret:write` (the public key is a read) |
| `delete` | `universe.secret:write` |

`universe.secret` is a beta scope. Limits: 500 secrets per universe, 120 requests per minute per key owner.
