.PHONY: all build test check doc bench

CARGO = cargo
BINARY = digitsum

all: test build

build:
	${CARGO} build --release
	cp target/release/${BINARY} .

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

