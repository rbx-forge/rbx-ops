# JSON output

Most reads, and some writes, take `--json`. Each command page documents its documents; the rules below hold for all of them.

- **One JSON document on stdout, and nothing else.** Notes, counts and warnings go to stderr, so `jq` reads the pipe while a person still reads the terminal.
- **`schema_version` comes first** where present, `1` today. A changed or removed field bumps it; an added field does not.
- **Ids are strings.** Universe, place and user ids exceed 2^53, which a JSON number would round. Money is a string too (`ads`).
- **Absent, not `null`.** A field that does not apply is left out, so `has("field")` is a usable test. Where `null` appears, it is a real value.
- **Stored values are nested, not escaped.** A data store or memory store value is JSON inside the document: `jq .value.coins` works.
- **Empty is a document.** Nothing found gives an empty array and exit `0`, never silence.
- **A failure prints nothing on stdout.** An empty stdout and a non-zero exit mean nothing happened.
- **No player data beyond what the human output shows.** Data store `users` and `attributes`, for instance, are never included.
