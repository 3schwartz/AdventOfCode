package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	codes := read.ReadData("day15")
	intCodes := coders.ParseIntCodes(codes)

	// Act
	_, movementCount := findOxygen(intCodes)

	// Assert
	if movementCount != 266 {
		t.Errorf("Wrong answer: %d", movementCount)
	}
}
