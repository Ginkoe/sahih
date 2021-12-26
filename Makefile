.PHONY: build clean

BIN_NAME = sahih
CARGO = $(shell which cargo)

build:
	$(CARGO) build