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

test20:
	cargo test --manifest-path=./day20/Cargo.toml --no-fail-fast --release
.PHONY:test20

# BENCH

bench5:
	cargo bench --bench benchmark --manifest-path=./day5/Cargo.toml -- --verbose
.PHONY:bench5

bench9:
	cargo bench --bench benchmark --manifest-path=./day9/Cargo.toml -- --verbose
.PHONY:bench9

bench18:
	cargo bench --bench benchmark --manifest-path=./day18/Cargo.toml -- --verbose

# Run

run5:
	cargo run --manifest-path=./day5/Cargo.toml --release

run14:
	cargo run --manifest-path=./day14/Cargo.toml --release

run15:
	cargo run --manifest-path=./day15/Cargo.toml --release

run17:
	cargo run --manifest-path=./day17/Cargo.toml --release	

run18:
	cargo run --manifest-path=./day18/Cargo.toml --release

run19:
	cargo run --manifest-path=./day19/Cargo.toml --release			

run21:
	cargo run --manifest-path=./day21/Cargo.toml --release
.PHONY:run21

.PHONY:run5 run14 run15 bench18 run19