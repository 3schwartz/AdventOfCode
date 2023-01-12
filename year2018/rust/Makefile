# TEST

test5:
	cargo test --manifest-path=./day5/Cargo.toml --no-fail-fast --lib --bins --tests --release
.PHONY:test5

test6:
	cargo test --manifest-path=./day6/Cargo.toml --no-fail-fast --release
.PHONY:test5

# BENCH

bench5:
	cargo bench --bench benchmark --manifest-path=./day5/Cargo.toml -- --verbose
.PHONY:bench5

bench9:
	cargo bench --bench benchmark --manifest-path=./day9/Cargo.toml -- --verbose
.PHONY:bench9

# Run

run5:
	cargo run --manifest-path=./day5/Cargo.toml --release
.PHONY:run5