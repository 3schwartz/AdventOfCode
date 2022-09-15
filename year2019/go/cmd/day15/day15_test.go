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
	_, oxygenDetectedDefinition := findOxygen(intCodes)

	// Assert
	if movementCount := oxygenDetectedDefinition.coder.GetMovementCount(); movementCount != 266 {
		t.Errorf("Wrong answer: %d", movementCount)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	codes := read.ReadData("day15")
	intCodes := coders.ParseIntCodes(codes)

	// Act
	allOxygen := findAllOxygen(intCodes)
	countToFillOxygen := fillOxygen(allOxygen)

	// Assert
	if countToFillOxygen != 274 {
		t.Errorf("Wrong answer: %d", countToFillOxygen)
	}
}
