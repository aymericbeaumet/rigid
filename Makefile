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
	cargo test -p tests

.PHONY: bench
bench:
	cargo bench --bench benches

.PHONY: bench-high-cpu
bench-high-cpu:
	sudo nice -n -20 make bench

.PHONY: clean
clean:
	cargo clean
