# rbx-ops analytics

Your experience's own metrics: players, retention, revenue per payer, funnels. For keeping history past Roblox's window, joining with your own data, and alerting. Needs `universe.analytics:read`.

```sh
rbx-ops analytics query --metric DailyActiveUsers --days 7 --env prod
rbx-ops analytics query --metric DailyActiveUsers --days 90 --csv --env prod > dau.csv
```

## rbx-ops analytics metrics

The metric names confirmed to work. Roblox publishes no list, and `--metric` takes any string.

```text
Visits                       Sessions started
DailyActiveUsers             Distinct players per day
MonthlyActiveUsers           Distinct players over the trailing month
D1Retention                  Share of new players returning the next day
D7Retention                  Share of new players returning within seven days
D30Retention                 Share of new players returning within thirty days
AverageRevenuePerPayingUser  ARPPU, in Robux
```

`--json` gives `{ schema_version, exhaustive: false, metrics: [{ name, description }] }`. Do not validate names against it.

## rbx-ops analytics query

```text
DailyActiveUsers (total)
  2026-07-27       1200.00
  2026-07-28       1245.00
```

| Flag | Meaning |
| --- | --- |
| `--metric <name>` | Required |
| `--days <n>` | How far back (default 30) |
| `--granularity <g>` | `one-minute`, `half-hour`, `one-hour`, `one-day` (default), `one-week`, `one-month`, `none` |
| `--breakdown <dim>` | One series per value of a dimension. Repeatable |
| `--filter <Dim=v[,v]>` | Keep only these values. Repeatable |
| `--csv` | `time,series,metric,value` for a spreadsheet or charting tool |
| `--json` | One JSON document. Not with `--csv` |

A wide range is queued by Roblox; the command waits up to a minute, then asks you to narrow `--days`. Some dimensions are filter-only, and Roblox refuses them as `--breakdown`.

#### `--json`

```json
{
  "schema_version": 1,
  "metric": "DailyActiveUsers",
  "granularity": "one-day",
  "days": 7,
  "start_time": "2026-07-27T09:12:04Z",
  "end_time": "2026-08-03T09:12:04Z",
  "breakdown": [],
  "filters": [],
  "queued": false,
  "totals": { "series": 1, "points": 3, "missing": 1 },
  "series": [
    {
      "label": "total",
      "dimensions": {},
      "points": [
        { "time": "2026-07-27T00:00:00+00:00", "value": 0 },
        { "time": "2026-07-28T00:00:00+00:00" },
        { "time": "2026-07-30T00:00:00+00:00", "value": 1288 }
      ]
    }
  ]
}
```

| Field | Meaning |
| --- | --- |
| `granularity` | As `--granularity` spells it |
| `start_time`, `end_time` | The range queried; end exclusive |
| `filters[]` | `{ dimension, operation, values }` |
| `queued` | Roblox made the command wait |
| `totals.missing` | Points with no value |
| `series[].dimensions` | The breakdown values of this series |

!!! warning "A missing value is not a zero"
    `"value": 0` is a measured zero. A point without `value`, or no point at all for a date, means Roblox reported nothing. Test with `has("value")`, and check `totals.missing` before averaging. Missing buckets are never filled in.

```sh
jq -e '.totals.missing == 0' dau.json > /dev/null && jq '[.series[].points[].value] | add / length' dau.json
```

## rbx-ops analytics dimensions

The values a dimension holds, to use with `--filter`.

```sh
rbx-ops analytics dimensions --metric DailyActiveUsers --dimension Platform --days 14 --env prod
```

| Flag | Meaning |
| --- | --- |
| `--metric`, `--dimension` | Required |
| `--days <n>` | How far back |

Opaque ids (funnel steps) are printed with their label; `--filter` takes the raw value.

## Funnels

The funnels your game logs with `AnalyticsService:LogFunnelStepEvent`: `FunnelUserTotalCount`, `FunnelUserStepCompletionRate`, `FunnelUserChurnRate`, `FunnelUserOverallCompletionRate`, their `FunnelStep*` session twins and the cohort metrics. They take `--granularity none` only, and `FunnelName` is filter-only:

```sh
rbx-ops analytics dimensions --metric FunnelUserTotalCount --dimension FunnelName --days 90 --env prod
rbx-ops analytics query --metric FunnelUserTotalCount \
  --filter FunnelName=Tutorial --breakdown FunnelStep \
  --granularity none --days 90 --env prod
```

An empty answer usually means the game never logged those events.
