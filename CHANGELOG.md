# Changelog

## [Unreleased]

First release as a standalone tool.

- `servers`, `analytics`, `ban`, `group`, `restart`, `data`, `memorystore`, `message`, `secret` and `ads`, with `user` hidden from `--help`.
- Every write is dry-run by default and needs `--apply`; `--env all` is refused.
- `--json` on every read, with one envelope across the tool: `schema_version` first. Field names are documented per command in `docs/`.
