package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("20_test")
	p, n := initPositions(input, 811589153)

	// Act
	c := mix(p, n, 10)
	sum := findIdxAfterZero(c, 1_000) + findIdxAfterZero(c, 2_000) + findIdxAfterZero(c, 3_000)

	// Assert
	if sum != 1623178306 {
		t.Error(sum)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("20_test")
	p, n := initPositions(input, 1)

	// Act
	c := mix(p, n, 1)
	sum := findIdxAfterZero(c, 1_000) + findIdxAfterZero(c, 2_000) + findIdxAfterZero(c, 3_000)

	// Assert
	if sum != 3 {
		t.Error(sum)
	}
}
