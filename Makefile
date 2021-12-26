.PHONY: build check ci clippy fmt lint test

BIN_NAME = sahih
CARGO = $(shell which cargo)

build:
	@$(CARGO) build


check:
	@$(CARGO) check --release

ci: lint check test

clippy:
	@$(CARGO) clippy

fmt:
	@$(CARGO) fmt

lint:
	@$(CARGO) fmt --all -- --check
	@echo "Lint OK 👌"

test:
	@$(CARGO) test -- --nocapture --test-threads=1 && echo "Tests OK 👌"