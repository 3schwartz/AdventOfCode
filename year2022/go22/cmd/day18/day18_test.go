package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("18_test")
	eKube, _ := createKube(input)

	// Act
	surface := eKube.findSurface()

	// Assert
	if surface != 64 {
		t.Error(surface)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("18_test")
	eKube, mm := createKube(input)

	// Act
	surface := eKube.findExteriorSurface(mm)

	// Assert
	if surface != 58 {
		t.Error(surface)
	}
}
