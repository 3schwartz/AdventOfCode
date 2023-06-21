# TEST

test5:
	cargo test --manifest-path=./day5/Cargo.toml --no-fail-fast --lib --bins --tests --release
.PHONY:test5

test6:
	cargo test --manifest-path=./day6/Cargo.toml --no-fail-fast --release
.PHONY:test5

test14:
	cargo test --manifest-path=./day14/Cargo.toml --no-fail-fast --release
.PHONY:test14

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

run14:
	cargo run --manifest-path=./day14/Cargo.toml --release

run15:
	cargo run --manifest-path=./day15/Cargo.toml --release

run17:
	cargo run --manifest-path=./day17/Cargo.toml --release	


.PHONY:run5 run14 run15