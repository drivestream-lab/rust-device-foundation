#!/usr/bin/env python3
"""Post-generation hook: print next steps after scaffold creation."""

DAEMON = "{{ cookiecutter.daemon_name }}"

print(
    f"""
{DAEMON} scaffold generated successfully.

Next steps:
  cd {DAEMON}
  make check && make test
  make run

Mount rust-device-rules as a git submodule at .cursor/rules (see README).
"""
)
