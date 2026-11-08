# rbx-ops restart

Move running servers onto the newest published version. Publishing alone restarts nothing: old servers run their code until they empty. Reading needs `universe:read`, launching `universe:write`.

```sh
rbx-ops restart forecast --env prod
rbx-ops restart launch --env prod --bleed-off 30 --apply
rbx-ops restart status --env prod
```

## rbx-ops restart forecast

How many players and servers a restart would hit **right now**, from Roblox's own forecast.

```text
PLACE                      PLAYERS HIT       INSTANCES HIT  NEWEST VERSION
55443322110099                   0/840                0/210  412
```

Servers already on the newest version are left alone, so "hit" can be far below the total. The count assumes no bleed-off; the real impact is lower.

## rbx-ops restart launch

Without `--apply`, shows the forecast and stops. With it, asks, then schedules. If every server is already current, nothing is asked.

| Flag | Meaning |
| --- | --- |
| `--bleed-off <minutes>` | Time before servers start closing, 1 to 240 (default 30). Players are not matched to those servers meanwhile, so most leave on their own |
| `--attribute <k=v>` | One entry of the dictionary servers receive, always a string. Repeatable |
| `--payload <json>` | The whole dictionary as JSON, for numbers, booleans or nesting. Not with `--attribute` |
| `--apply` | Launch |
| `--yes` | Skip the confirmation |

The dictionary reaches the game through `ServerRestartScheduled`, so it can warn players or teleport them out during the bleed-off:

```lua
game.ServerRestartScheduled:Connect(function(restartTime, source, attributes)
    -- attributes: what --attribute or --payload sent
end)
```

```sh
rbx-ops restart launch --env prod --payload '{"reason":"hotfix","urgency":3}' --apply
```

It must be a JSON object of at most 500 bytes; both are checked before any request.

!!! warning "Never run for real end to end"
    `launch --apply` is covered by tests, not by a real restart, and whether the attributes arrive intact has not been observed. For the first real launch: run `forecast`, pick a quiet hour, use a long bleed-off.

## rbx-ops restart status

The current restart: `DELAYING` (bleed-off), `RESTARTING`, `SUCCEEDED`.
