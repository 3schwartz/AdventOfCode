clippy:
	cargo clippy --all-targets --all-features
.PHONY:clippy

format:
	cargo fmt --all
.PHONY:format

clean: format clippy
.PHONY:cleans

test:
	cargo test --manifest-path=./day12/Cargo.toml --no-fail-fast --release

run:
	cargo run --manifest-path=./day12/Cargo.toml --release
.PHONY:test
