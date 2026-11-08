# Changelog

## [Unreleased]

## [0.1.0] - 2026-11-08

First release as a standalone tool.

- `servers`, `analytics`, `ban`, `group`, `restart`, `data`, `memorystore`, `message`, `secret` and `ads`, with `user` hidden from `--help`.
- Every write is dry-run by default and needs `--apply`; `--env all` is refused.
- `--json` on most reads, and on some writes when paired with `--yes`. Each command's page in `docs/` lists which subcommands take it and the fields of each document.
