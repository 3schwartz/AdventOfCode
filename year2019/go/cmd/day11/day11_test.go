package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	codes := read.ReadData("day11")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.PaintHullIntCoder{}

	// Act
	outputs := intCoder.PaintHull(intCodes)

	// Assert
	if length := len(outputs); length != 2415 {
		t.Errorf("Wrong length got from result: %d", length)
	}
}
