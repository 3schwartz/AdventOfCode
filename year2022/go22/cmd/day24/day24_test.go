package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("24_test")

	// Act
	steps := findShortestSteps(input, 1)

	// Assert
	if steps != 18 {
		t.Error(steps)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("24_test")

	// Act
	steps := findShortestSteps(input, 2)

	// Assert
	if steps != 54 {
		t.Error(steps)
	}
}
