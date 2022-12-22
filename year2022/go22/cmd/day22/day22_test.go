package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	// input := io.ReadData("22_test")

	// Act

	// Assert
	// // if sum != XXX {
	// // 	t.Error(sum)
	// // }
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("22_test")
	jungleMap, instructions, yMax, xMax := createMap(input)

	// Act
	password := jungleMap.followInstructions(instructions, yMax, xMax)

	// Assert
	if password != 6032 {
		t.Error(password)
	}
}
