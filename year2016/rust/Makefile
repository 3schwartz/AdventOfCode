run9:
	cargo run --manifest-path=./day9/Cargo.toml --release
.PHONY:run9

clippy:
	cargo clippy --all-targets --all-features
.PHONY:clippy

format:
	cargo fmt --all
.PHONY:format

clean: format clippy
.PHONY:clean
