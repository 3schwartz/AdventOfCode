.DEFAULT_GOAL := build

fmt:
	go fmt ./...
.PHONY:fmt

vet: fmt
	go vet ./...
.PHONY:vet

build:
	go build ./...
.PHONY:build

build-only:
	go build ./...

test: build-only
	go test ./...

# Tests
test3:
	go test ./cmd/day3 -v

test4:
	go test ./cmd/day4

test5:
	go test ./cmd/day5

test6:
	go test ./cmd/day6

test7:
	go test ./cmd/day7 -v

test8:
	go test ./cmd/day8 -v

test10:
	go test ./cmd/day10

test11:
	go test ./cmd/day11

test12:
	go test ./cmd/day12	

test13:
	go test ./cmd/day13

test14:
	go test ./cmd/day14

test15:
	go test ./cmd/day15

test16:
	go test ./cmd/day16

test17:
	go test ./cmd/day17	

# Benchmarks
bench3:
	go test ./cmd/day3 -v -bench=. -benchmem -run=^a

bench4:
	go test ./cmd/day4 -bench=. -benchmem -run=^a	

bench7:
	go test ./cmd/day7 -bench=. -benchmem -run=^a

bench10:
	go test ./cmd/day10 -bench=. -benchmem -run=^a	

bench12:
	go test ./cmd/day12 -bench=. -benchmem -run=^a	

bench14:
	go test ./cmd/day14 -bench=. -benchmem -run=^a	