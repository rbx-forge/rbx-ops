# rbx-ops ads

Launch and steer Roblox ad campaigns, mainly to test icons: one campaign per image, identical otherwise. Scopes: `ad.campaign:read`, `ad.campaign:write`, and `ad.billing:read` for `accounts`.

```sh
rbx-ops ads launch --creative 18234567890 --creative 18234567891 \
  --name "icon test" --budget 25 --days 14 --env prod          # cost shown, nothing sent
rbx-ops ads status c1 c2
rbx-ops ads pause --name "icon test" --apply
```

!!! warning "This spends real money"
    Every write is a dry run without `--apply`, and asks before acting. `--yes` removes that question: together with `--apply` it is an unattended charge. A campaign cannot be refunded.

Roblox ships this API as an experiment, and it has **no reporting**: read impressions, clicks and spend in Ads Manager.

## Testing an image

Ads Manager reports per campaign, so testing three images needs three campaigns: `launch` creates one per `--creative`, each named `<name> [<asset id>]` so its row can be traced back to the image. Identical campaigns compete a little for the same impressions, which hurts every variant equally and keeps the ranking.

For **thumbnails**, prefer the free A/B test in the Creator Dashboard. The **icon** has no such test, which is where this helps.

Running the same `launch` twice finds the existing campaigns instead of buying more (the idempotency key comes from the campaign's definition). Change `--name` for a second test.

## rbx-ops ads launch

| Flag | Meaning |
| --- | --- |
| `--creative <asset_id>` | One image per campaign; repeat. On a terminal, omit it to pick from a list (at least two) |
| `--name` | Base name |
| `--budget` | Dollars **per campaign**: five variants at 25 is 125 |
| `--budget-type` | `DAILY` or `LIFETIME` |
| `--days` | Duration |
| `--start` | RFC 3339 (default: once review clears) |
| `--payment` | `CREDIT_CARD`, `ADS_CREDIT` or `INVOICE` (see `options`) |
| `--country`, `--age`, `--device` | Narrow the audience; repeatable |
| `--apply`, `--yes` / `-y` | Create; skip the confirmation |

New campaigns start `IN_REVIEW`. `REJECTED` comes with reasons.

## Changing campaigns

| Command | Does |
| --- | --- |
| `pause [id]`, `resume [id]`, `cancel [id]` | `--name <prefix>` acts on every campaign whose name starts with it. No id on a terminal: pick from a list |
| `budget [id] --amount <usd>` | An increase applies at once; a decrease on a running campaign at the next midnight of the account's time zone |
| `rename <id> --name <name>` | Delivery and budget untouched |

All take `--apply` and `--yes`. Cancelled and completed campaigns are skipped (Roblox refuses edits to them).

## Reading

| Command | Does |
| --- | --- |
| `list` | Every campaign. `--json` |
| `get <id>` | One campaign. `--json` gives `{ campaign: {...} }`, the same object as in `list` |
| `status <id>...` | Delivery state of several. `--json` |
| `creatives` | Images usable as creatives, with moderation state. `--archived` for archived ones |
| `universes` | Experiences this account may advertise |
| `accounts` | Billing accounts the key can spend from |
| `options` | Formats, objectives, payment types, targeting, eligibility |

`list --json`:

```json
{
  "schema_version": 1,
  "totals": { "returned": 1, "active": 1 },
  "campaigns": [
    {
      "id": "c_8f3a91",
      "name": "icon test [18234567890]",
      "status": "ACTIVE",
      "delivery_status": "SERVING",
      "delivery_status_reasons": [],
      "budget": { "amount_micros": "25500000", "amount_usd": "25.50", "type": "DAILY" },
      "target_universe_id": "5544332211",
      "creative_asset_ids": ["18234567890"]
    }
  ]
}
```

- **Money is a string.** Compute on `amount_micros`, print `amount_usd`. `budget` is absent when Roblox gives none.
- **`status` is not `delivery_status`.** `ACTIVE` but `IN_REVIEW` spends nothing.

`status --json` gives `{ totals: { requested, returned, failed }, statuses: [...], failures: [{ id, reason }] }`; ids Roblox could not answer for are in `failures`, never in `statuses`.

```sh
rbx-ops ads list --json | jq -r '.campaigns[] | select(.status == "ACTIVE" and .delivery_status != "SERVING") | .name'
```
