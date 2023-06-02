.PHONY: all sstat sstatc test sstat-test sstatc-test lint fmt

TOOLCHAIN = nightly
PROFILE = dev
CLIPPY_FLAGS = -D warnings -W unreachable-pub -W bare-trait-objects

# BUILD

all: lint
	cargo +$(TOOLCHAIN) build --profile $(PROFILE) --workspace

sstat: lint
	cargo +$(TOOLCHAIN) build --profile $(PROFILE) --package sstat

sstatc: lint
	cargo +$(TOOLCHAIN) build --profile $(PROFILE) --package sstatc

# TEST

test: lint
	cargo +$(TOOLCHAIN) test --profile $(PROFILE) --workspace

sstat-test: lint
	cargo +$(TOOLCHAIN) test --profile $(PROFILE) --package sstat

sstatc-test: lint
	cargo +$(TOOLCHAIN) test --profile $(PROFILE) --package sstatc

# FORMAT

lint: fmt
	cargo +$(TOOLCHAIN) clippy --profile $(PROFILE) -- $(CLIPPY_FLAGS)

fmt:
	cargo +$(TOOLCHAIN) fmt
