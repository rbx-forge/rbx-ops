# Changelog

## [Unreleased]

### Fixed

- `data ordered set`, `increment` and `delete` are dry runs until `--apply`, like every other write. They wrote as soon as the question was answered, and `--yes` alone wrote at once.
- `group rank` and `unrank` refuse an `--env` that `rbxplace.toml` does not have, `all`, and a group of envs. Each fell back to the top-level `[owner]`, so a typo acted on another group than the one meant.
- A write that is not idempotent is no longer sent again after a 5xx or a timeout: `data increment`, `data ordered increment`, `restart launch` and `message`. Roblox may have applied it before the answer was lost, and a second one added the amount, launched the restart or delivered the message twice. The error says the request may have landed and how to check. A 429 is still retried: it is a refusal before anything happened.

## [0.1.1] - 2026-11-09

### Security

- Release archives carry a build provenance attestation: `gh attestation verify <zip> --repo rbx-forge/rbx-ops`. The build jobs run with read access only, and the publishing job alone may write.

### Fixed

- `servers versions`, `list` and `logs` take `--universe-id` with `--place-id`, like every other command, and no longer refuse everything but `--env`.

## [0.1.0] - 2026-11-08

First release as a standalone tool.

- `servers`, `analytics`, `ban`, `group`, `restart`, `data`, `memorystore`, `message`, `secret` and `ads`, with `user` hidden from `--help`.
- Every write is dry-run by default and needs `--apply`; `--env all` is refused.
- `--json` on most reads, and on some writes when paired with `--yes`. Each command's page in `docs/` lists which subcommands take it and the fields of each document.
