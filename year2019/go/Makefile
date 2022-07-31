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
	go test ./... -v

# Tests
test3:
	go test ./cmd/day3 -v

test4:
	go test ./cmd/day4

test7:
	go test ./cmd/day7 -v

# Benchmarks
bench3:
	go test ./cmd/day3 -v -bench=. -benchmem -run=^a

bench4:
	go test ./cmd/day4 -bench=. -benchmem -run=^a	

bench7:
	go test ./cmd/day7 -bench=. -benchmem -run=^a