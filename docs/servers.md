# rbx-ops servers

Live and terminated servers, and the logs of one that stopped. Needs `universe:read`.

```sh
rbx-ops servers versions --env prod
rbx-ops servers list --env prod --status crashed --full
rbx-ops servers logs aba9aeae-bc55-49c8-bb0e-6363ee6ba820 --version 407 --env prod
```

!!! warning "Roblox keeps 30 days"
    Terminated servers and their logs are discarded after thirty days. Export what you want to keep (`--csv` or `--json`) on a schedule.

## rbx-ops servers versions

The place versions that have servers, newest first. Roblox lists servers per version only, so this is how you find one. The `*` marks the default the other commands use.

```text
place versions with servers (newest first)
  * 412
    407
```

`--json`:

```json
{ "schema_version": 1, "default_place_version": "412", "place_versions": ["412", "407"] }
```

`default_place_version` is absent when nothing ran in thirty days.

## rbx-ops servers list

```text
STATUS      JOB                  UPTIME   MEMORY     FPS  PLAYERS
active      9680282c                16s   569 MB      60  2/7
crashed     aba9aeae            10h 53m  1111 MB      60  0/7

2 rows for place version 412, 24610 exist (--limit 2 reached)
```

| Flag | Meaning |
| --- | --- |
| `--version <n>` | Place version (default: newest with servers) |
| `--status <s>` | `active`, `shut_down`, `restarted`, `roblox_restarted`, `crashed`, `out_of_memory`, `moderated` |
| `--limit <n>` | Rows to fetch (default 50). A busy version has tens of thousands |
| `--full` | Whole job ids, which `logs` needs |
| `--csv` | Every field Roblox returns, as CSV |
| `--json` | One JSON document. Not with `--csv` or `--full` |

`crashed` and `out_of_memory` are highlighted and counted. `FPS` shows `-` when Roblox measured nothing, `0` when it measured zero.

!!! warning "A warning above the table means missing rows"
    Roblox can answer `200` while reporting it failed to read one of its sources. The page is then partial and any rate computed from it is wrong. The warning goes to stderr, and `--json` sets `partial`.

#### `--json`

```json
{
  "schema_version": 1,
  "place_version": "412",
  "partial": false,
  "limit": 50,
  "limit_reached": false,
  "totals": { "returned": 1, "failed": 1, "available": 24610 },
  "servers": [
    {
      "job_id": "aba9aeae-bc55-49c8-bb0e-6363ee6ba820",
      "status": "crashed",
      "failure": true,
      "place_id": "234567890123456",
      "place_version": "412",
      "engine_version": "0.700.0.7000000",
      "create_time": "2025-08-14T02:53:11Z",
      "termination_time": "2025-08-14T13:46:53Z",
      "uptime_seconds": 39222,
      "memory_bytes": 1165064601,
      "frame_rate": 60.0,
      "occupancy": 0,
      "max_occupancy": 7,
      "full": false,
      "shut_down": true,
      "type": 1,
      "player_count": 0,
      "player_ids": []
    }
  ]
}
```

| Field | Meaning |
| --- | --- |
| `partial` | Roblox reported a fetch error: rows are missing |
| `limit_reached` | Stopped at `--limit`; there are more |
| `totals.available` | Servers for this version before filtering. Absent if Roblox did not say |
| `servers[].failure` | `true` for `crashed` and `out_of_memory` |
| `servers[].status` | As `--status` spells it, or `unknown` |
| `servers[].termination_time` | Absent on a live server |
| `servers[].frame_rate` | Absent when never measured |
| `servers[].type` | Roblox's raw enum, unnamed |
| `servers[].player_ids` | Dropped from CSV for width. Absent if Roblox sent none |

```sh
rbx-ops servers list --env prod --limit 500 --json \
  | jq -r '.servers[] | select(.failure) | "\(.termination_time) \(.job_id)"'
```

## rbx-ops servers logs

`rbx-ops servers logs <job_id>`

What one server logged. Stack traces are never truncated.

```text
13:46:53  error    ServerScriptService.Gameplay.RoundService:88: attempt to index nil with 'Name'
              Stack Begin
              Script 'ServerScriptService.Gameplay.RoundService', Line 88
              Stack End
```

| Flag | Meaning |
| --- | --- |
| `--version <n>` | The version the server ran (default: newest). **Must match**: a job from 407 queried on 412 returns nothing |
| `--severity <s>` | `output`, `info`, `warn`, `error` |
| `--limit <n>` | Lines (default 200) |
| `--csv` | CSV, stack traces quoted |
| `--json` | One JSON document. Not with `--csv` |

#### `--json`

One document per run, not JSON Lines (`jq -c '.lines[]'` converts it).

```json
{
  "schema_version": 1,
  "job_id": "aba9aeae-bc55-49c8-bb0e-6363ee6ba820",
  "place_version": "407",
  "limit": 200,
  "limit_reached": false,
  "totals": { "returned": 1, "errors": 1 },
  "lines": [
    {
      "time": "2025-08-14T13:46:53.481Z",
      "severity": "error",
      "severity_code": 3,
      "error": true,
      "message": "ServerScriptService.Gameplay.RoundService:88: attempt to index nil with 'Name'",
      "stack_trace": "Stack Begin\nScript 'ServerScriptService.Gameplay.RoundService', Line 88\nStack End",
      "job_id": "aba9aeae-bc55-49c8-bb0e-6363ee6ba820",
      "place_version": "407"
    }
  ]
}
```

`severity_filter` appears when `--severity` was given. `time`, `severity_code`, `message` and `stack_trace` are absent when the line had none. No logs gives an empty `lines` and exit 0.

```sh
rbx-ops servers logs <job_id> --version 407 --env prod --severity error --json \
  | jq -r '.lines[] | select(has("stack_trace")) | .stack_trace'
```
