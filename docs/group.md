# rbx-ops group

A group's roles, its members, and moving members between roles. Open Cloud cannot create, rename or delete roles: do that in the Creator Hub. Every group endpoint is beta.

```sh
rbx-ops group roles
rbx-ops group members Moderator
rbx-ops group rank builderman Moderator --apply
```

**Which group:** `--group <id>`, or else `[owner]` in `rbxplace.toml` (an env's `[<env>.owner]` with `--env`). An owner that is a user is refused.

| Subcommand | Scope |
| --- | --- |
| `roles`, `member`, `members` | `group:read` |
| `rank`, `unrank` | `group:read` and `group:write` |

## rbx-ops group roles

Every role, lowest rank first.

```text
 RANK  NAME                            MEMBERS  ID
    0  Guest                                 -  2788100
    1  Member                              412  2788101
   50  Moderator                             6  2788109
  255  Owner                                 1  2788115
```

`-` means Roblox gave no count (the guest role), not zero. `--json` prints each role as Roblox sends it, `permissions` included, under `group_id`.

!!! warning "Missing `permissions` means unknown, not none"
    Roblox returns a role's permissions only for the guest role, the key creator's own role, and every role if the key creator owns the group. Treat a missing `permissions` as unknown.

## rbx-ops group member

`rbx-ops group member <user>`

Every role a member holds, highest first.

## rbx-ops group members

`rbx-ops group members [role]`

```sh
rbx-ops group members Moderator --limit 500 --json
```

| Flag | Meaning |
| --- | --- |
| `--limit <n>` | Members to return (default 100) |
| `--cursor <token>` | Continue where the previous run stopped |
| `--json` | `source`, `members[]` (`user_id`, `username`, `display_name`, `roles`), `scanned`, `next_cursor` (`null` at the end) |

Paged, never exhaustive: a run reads at most 2,000 members and prints the command that continues. A cursor is valid for the same group, role and listing only.

With a role and a [session cookie](reference/configuration.md#the-studio-cookie), the role's holders are listed directly (the Creator Hub's own listing, outside Roblox's documented API); members' other roles are then not reported. Without a session, every member is read and filtered here, so a rare role in a large group can take several runs.

## rbx-ops group rank and unrank

```text
rbx-ops group rank <USER> <ROLE> [--apply] [--yes]
rbx-ops group unrank <USER> <ROLE> [--apply] [--yes]
```

`<USER>` takes what [`ban`](ban.md#naming-a-player) takes. `<ROLE>` is an id or a name (case-insensitive; `name:` for a name made of digits). A name shared by two roles is refused, with both ids.

`rank` **adds** a role; the member keeps the others. Without `--apply`, everything is resolved and nothing sent; with it, you are asked to confirm (`--yes` skips). The member must already be in the group.
