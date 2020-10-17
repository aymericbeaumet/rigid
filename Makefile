.PHONY: all
all: clean build ci bench

.PHONY: build
build:
	cargo build

.PHONY: ci
ci: lint test

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
	cargo test --benches

.PHONY: bench
bench:
	@rm -rf benches/target/criterion
	cargo bench --bench benches

.PHONY: clean
clean:
	cargo clean
