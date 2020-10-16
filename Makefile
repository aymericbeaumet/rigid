.PHONY: all
all: clean build ci

.PHONY: build
build:
	cargo build

.PHONY: ci
ci: lint test bench

.PHONY: lint
lint:
	cargo clippy -- \
		-W clippy::cargo \
		-W clippy::nursery \
		-W clippy::pedantic \
		-A clippy::cargo-common-metadata \
		-A clippy::missing-errors-doc

.PHONY: test
test:
	cargo test

.PHONY: bench
bench:
	cargo bench

.PHONY: clean
clean:
	cargo clean
