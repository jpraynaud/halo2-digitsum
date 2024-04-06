.PHONY: all build test check doc bench

CARGO = cargo

all: test build

build:
	${CARGO} build --release

run:
	${CARGO} run --release

test:
	${CARGO} test --features default

check:
	${CARGO} check --release --all-features --all-targets
	${CARGO} clippy --release --all-features --all-targets
	${CARGO} fmt --check

doc:
	${CARGO} doc --no-deps --open --features default

bench:
	${CARGO} bench --features default --verbose
