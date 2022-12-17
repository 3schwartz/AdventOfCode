package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	jetPattern := io.ReadData("17_test")

	// Act
	maxY := findRockHeight(1_000_000_000_000, jetPattern, false)

	// Assert
	if maxY != 1_514_285_714_288 {
		t.Error(maxY)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	jetPattern := io.ReadData("17_test")

	// Act
	maxY := findRockHeight(2022, jetPattern, false)

	// Assert
	if maxY != 3068 {
		t.Error(maxY)
	}
}
