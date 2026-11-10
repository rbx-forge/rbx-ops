# rbx-ops message

Send a MessagingService message to every running server, from outside the game: a deploy step or a cron job telling servers something changed. Needs `universe-messaging-service:publish`, which most keys lack.

```sh
rbx-ops message --topic cache --message reload --env prod            # described, not sent
rbx-ops message --topic cache --payload '{"key":"rotation"}' --apply --env prod
```

| Flag | Meaning |
| --- | --- |
| `--topic` | What the game passes to `MessagingService:SubscribeAsync` |
| `--message <text>` | The body, sent as is |
| `--payload <json>` | A JSON body, validated before sending (the game decodes it with `HttpService:JSONDecode`) |
| `--apply` | Send |
| `--json` | One receipt on stdout |

The body must be 1 to 1114 bytes (Roblox says 1 KB; 1114 is the measured limit), checked before sending. Send a reference, such as a [memory store](memorystore.md) key, rather than the data itself.

`--json`:

```json
{ "schema_version": 1, "topic": "cache", "universe_id": "5544332211", "bytes": 6, "applied": true }
```

`applied` is `false` on a dry run, which then also includes `message`. A failed run prints nothing on stdout.

!!! warning "Success means accepted, not received"
    Roblox reports no delivery. An experience with no servers accepts a message exactly like a busy one. If you need confirmation, have servers write back, to a memory store for instance.

A message Roblox does not confirm (a 5xx, a timeout) is not sent again: it may have been published, and a second one would reach every listening server twice. The error says so. Make the handler safe to run twice if the sender may retry by hand.
