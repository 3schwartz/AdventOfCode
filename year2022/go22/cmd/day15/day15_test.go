package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("15_test")
	cave := createCave(input)

	// Act
	count := cave.findDistressSignal(20)

	// Assert
	if count != 56000011 {
		t.Error(count)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("15_test")
	cave := createCave(input)

	// Act
	count := cave.findNonPossiblePositionsAtRow(10)

	// Assert
	if count != 26 {
		t.Error(count)
	}
}
