package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	codes := read.ReadData("day13")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.ArcadeIntCoder{}

	// Act
	state := intCoder.PlayArcade(intCodes)

	// Assert
	if state.BlockCount != 344 {
		t.Errorf("Wrong answer: %d", state.BlockCount)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	codes := read.ReadData("day13")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.ArcadeIntCoder{}

	// Act
	intCodes[0] = 2
	state := intCoder.PlayArcade(intCodes)

	// Assert
	if state.BlockCount != 344 {
		t.Errorf("Wrong count: %d", state.BlockCount)
	}
	if state.TotalScore != 17336 {
		t.Errorf("Wrong score: %d", state.TotalScore)
	}
}
