package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("14_test")

	// Act
	cave, yMax := createCave(input)
	sand := getFillCount(cave, yMax)

	// Assert
	if sand != 93 {
		t.Error(sand)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("14_test")

	// Act
	cave, yMax := createCave(input)
	sand := getSandCount(cave, yMax)

	// Assert
	if sand != 24 {
		t.Error(sand)
	}
}
