# rbx-ops

Operate a live Roblox experience from the command line: its servers, players, data, the messages and secrets its servers read, and its ad campaigns.

```sh
rbx-ops servers list --env prod --status crashed   # what crashed, and why
rbx-ops ban add builderman --reason "exploit" --duration 7d --env prod
rbx-ops data --datastore PlayerData get Player_156 --env prod
```

[Quick start](quickstart.md){ .md-button .md-button--primary } [Safety model](reference/safety.md){ .md-button }

**Dry run by default.** Every write describes itself and stops until you pass `--apply`.

**Scriptable.** `--json` on reads gives one stable document; failures print nothing on stdout.

**One key per job.** Each command needs a narrow Open Cloud scope, listed on its page.

## Install

=== "mise"

    ```sh
    mise use --pin github:rbx-forge/rbx-ops
    ```

=== "Rokit"

    ```sh
    rokit add rbx-forge/rbx-ops
    ```

Archives for every platform are on the [releases page](https://github.com/rbx-forge/rbx-ops/releases).

## Commands

| Command | What it does |
| --- | --- |
| [`servers`](servers.md) | Live and terminated servers, and a crashed server's logs |
| [`analytics`](analytics.md) | Players, retention, revenue, funnels. CSV for charting |
| [`ban`](ban.md) | Inspect and change player restrictions |
| [`group`](group.md) | List a group's roles and members, move members between roles |
| [`restart`](restart.md) | Forecast and launch a rolling server restart |
| [`data`](data.md) | Read, overwrite, copy and recover data store entries; leaderboards |
| [`memorystore`](memorystore.md) | Write values servers read through `MemoryStoreService` |
| [`message`](message.md) | Send a MessagingService message to every server |
| [`secret`](secret.md) | Secrets `HttpService:GetSecret` reads, sent encrypted |
| [`ads`](ads.md) | Launch and steer ad campaigns. Spends money |
| [`user`](user.md) | Username to id and back |

Nothing here reconciles a file you commit: banning a player is a reaction to last night, not a desired state.

---

A community tool under [MPL-2.0](https://github.com/rbx-forge/rbx-ops/blob/main/LICENSE), not affiliated with Roblox Corporation. Source and changelog on [GitHub](https://github.com/rbx-forge/rbx-ops).
