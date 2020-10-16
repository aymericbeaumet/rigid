.PHONY: all
all: clean build ci

.PHONY: build
build:
	cargo build

.PHONY: ci
ci: lint test bench

.PHONY: lint
lint:
	cargo clippy --all-targets -- \
		-W clippy::cargo \
		-W clippy::nursery \
		-W clippy::pedantic \
		-A clippy::cargo-common-metadata \
		-A clippy::missing-errors-doc

.PHONY: test
test:
	cargo test --all-targets

.PHONY: bench
bench:
	cargo bench --all-targets

.PHONY: clean
clean:
	cargo clean
