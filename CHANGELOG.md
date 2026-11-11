# Changelog

## [Unreleased]

### Security

- **The API key no longer follows a redirect to another host.** Every command stops at a redirect that leaves the host it called and reports it with its status. A request redirected elsewhere used to go there with its `x-api-key` header.

### Fixed

- **A write answered after a redirect says it may have landed.** `data increment`, `data ordered increment`, `restart launch` and `message` followed a 302 as a GET, and a 4xx from wherever it pointed read as a refusal. The error now says the write may have gone through and how to check, as after a 5xx, and so does a redirect to another host.

## [0.2.1] - 2026-11-10

### Fixed

- `data increment`, `data ordered increment`, `restart launch` and `message` are sent again after a failure to connect, which sent nothing. They are still not sent again once the request has left.
- A wait before a retry says what Roblox answered: a rate limit after a 429, the status after a 5xx. Every wait read "Rate limited by Roblox".

## [0.2.0] - 2026-11-10

### Changed

- **`data ordered set`, `increment` and `delete` are dry runs until `--apply`**, like every other write. They wrote as soon as the question was answered, and `--yes` alone wrote at once. A script that calls one of them now needs `--apply`.

### Fixed

- `group rank` and `unrank` refuse an `--env` that `rbxplace.toml` does not have, `all`, and a group of envs. Each fell back to the top-level `[owner]`, so a typo acted on another group than the one meant.
- A write that is not idempotent is no longer sent again after a 5xx or a timeout: `data increment`, `data ordered increment`, `restart launch` and `message`. Roblox may have applied it before the answer was lost, and a second one added the amount, launched the restart or delivered the message twice. The error says the request may have landed and how to check, and a 2xx whose answer cannot be read says the write is applied. A 429 is still retried: it is a refusal before anything happened.

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
