package main

import (
	"advent2022/pkg/io"
	"testing"
)

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
