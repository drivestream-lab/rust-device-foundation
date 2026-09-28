# Agent notes — {{ cookiecutter.daemon_name }}

- Mount **rust-device-rules** @ **v0.1.0** under `.cursor/rules`.
- Keep MQTT / Runtime API / OTA / storage topology in **product ADRs**, not chassis crates.
- Prefer `HostBackend` for OS calls; do not scatter `Command` in the binary.
- Day-one gate: `make check && make test`.
