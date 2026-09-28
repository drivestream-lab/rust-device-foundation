# {{ cookiecutter.daemon_name }}

{{ cookiecutter.daemon_description }}

Generated from [`rust-device-foundation`](https://github.com/drivestream-lab/rust-device-foundation).

## Constitution

```bash
git submodule add https://github.com/drivestream-lab/rust-device-rules.git .cursor/rules
cd .cursor/rules && git checkout v0.1.0
```

## Develop

```bash
make check   # fmt + clippy -D warnings + cargo check
make test
make run     # short reconcile + local IPC demo
```

Env:

| Var | Default | Meaning |
|-----|---------|---------|
| `DAEMON_TICKS` | `3` | Reconcile ticks before exit |
| `DAEMON_IPC_PATH` | `$TMPDIR/{{ cookiecutter.daemon_name }}.sock` | UDS path |
| `DAEMON_SKIP_IPC` | unset | Set `1` to skip IPC demo |
| `RUST_LOG` | `info` | tracing filter |

## Layout

```
crates/core/            traits, reconcile, local IPC stub
crates/platform-mac/    macOS HostBackend
crates/platform-linux/  Linux HostBackend
crates/bin/             {{ cookiecutter.daemon_name }} binary
deploy/systemd/         unit template
docs/specification/     product / adr / as-built
```

Product concerns (sole MQTT gateway, Runtime API schema, storage classes) belong in ADRs — not this chassis.
