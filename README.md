# rust-device-foundation

**Cookiecutter chassis for long-running Rust device daemons** — day-one-green
Cargo workspace with core + platform backends, local IPC stub, systemd unit
template, and pin instructions for [`rust-device-rules`](https://github.com/drivestream-lab/rust-device-rules).

| | |
|---|---|
| **Template engine** | Cookiecutter |
| **Version** | see [`VERSION`](VERSION) (currently **0.1.1**) · [CHANGELOG](CHANGELOG.md) |
| **License** | [MIT](LICENSE) |
| **Constitution** | Pin **`rust-device-rules`** @ **v0.1.0** at `.cursor/rules` |
| **Stack** | Rust 2021 · Tokio · tracing |

## What the chassis guarantees

1. **Shape** — workspace: `core` (traits + reconcile loop stub) + `platform-mac` / `platform-linux` + binary crate
2. **Constitution alignment** — consumers mount `rust-device-rules`
3. **Day-one-green** — `make check` (fmt + clippy) and `make test` pass on macOS / linux hosts
4. **No product topology** — sole MQTT gateway, storage class enums, Runtime API catalogs stay in product ADRs

## Usage

```bash
pip install cookiecutter
cookiecutter gh:drivestream-lab/rust-device-foundation
# or local:
cookiecutter /path/to/rust-device-foundation

cd <daemon_name>
make check && make test
make run
```

## Options

| Option | Meaning |
|--------|---------|
| `daemon_name` | Repo / binary / systemd unit name (kebab-case) |
| `daemon_description` | Human-readable description |

## Related

| Repo | Role |
|------|------|
| [`rust-device-rules`](https://github.com/drivestream-lab/rust-device-rules) | Constitution |
| `edge-agent-foundation` | Python Edge Agent chassis (different craft) |

## License

MIT — see [LICENSE](LICENSE).
