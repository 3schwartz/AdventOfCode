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