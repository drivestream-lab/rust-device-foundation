# Changelog

All notable changes to `rust-device-foundation` are documented here.

---

## v0.1.1

### Fixed

- Stop gitignoring `.cursor/rules` (Launchpad mounts constitution as a submodule).
- Ignore harness skill hubs (`.harness/skills/`, `.agents/`, `.claude/`).

## v0.1.0

### Summary

Initial cookiecutter: Cargo workspace with `core` + `platform-mac` +
`platform-linux` + binary, systemd unit template, Makefile check/test/run,
day-one-green on host targets.

### Migration guide

- Greenfield: `cookiecutter` this repo; pin `rust-device-rules` v0.1.0
