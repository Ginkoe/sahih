.PHONY: build check ci clippy fmt lint test release install

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

release:
	@$(CARGO) build --release

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)