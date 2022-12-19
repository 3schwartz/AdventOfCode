package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("19_test")
	blueprints := createBlueprints(input)

	// Act
	sum := findSum(blueprints)

	// Assert
	if sum != 33 {
		t.Error(sum)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("19_test")
	blueprints := createBlueprints(input)

	// Act
	mul := findMul(blueprints, 2)

	// Assert
	if mul != 56*62 {
		t.Error(mul)
	}
}
