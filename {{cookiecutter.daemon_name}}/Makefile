.PHONY: check test run fmt clippy clean

CARGO ?= cargo

fmt:
	$(CARGO) fmt --all

clippy:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

check: fmt clippy
	$(CARGO) check --workspace

test:
	$(CARGO) test --workspace

run:
	DAEMON_TICKS=3 $(CARGO) run -p {{ cookiecutter.daemon_name | replace('-', '_') }}

clean:
	$(CARGO) clean
