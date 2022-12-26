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

# Tests
test: build-only
	go test ./...

test2:
	go test ./cmd/day2 -v

test7:
	go test ./cmd/day7 -v

test8:
	go test ./cmd/day8 -v

test9:
	go test ./cmd/day9 -v

test10:
	go test ./cmd/day10 -v	

test11:
	go test ./cmd/day11 -v		

test12:
	go test ./cmd/day12 -v			

test13:
	go test ./cmd/day13 -v				

test14:
	go test ./cmd/day14 -v	

test24:
	go test ./cmd/day24 -v

test25:
	go test ./cmd/day25 -v

# Benchmarks
bench16:
	go test ./cmd/day16 -v -bench=. -benchmem -run=^a