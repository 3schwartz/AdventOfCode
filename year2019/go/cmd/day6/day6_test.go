package main

import (
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	lines := readData()
	orbitCalculator := newOrbitCountCalculator(lines)

	// Act
	orbitCount := orbitCalculator.getOrbitCount()

	// Assert
	if orbitCount != 142497 {
		t.Error("Wrong orbit count", orbitCount)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	lines := readData()
	orbitCalculator := newOrbitMinimalDistanceCalculator(lines)

	// Act
	minimalDistance, err := orbitCalculator.getMinimalDistance("YOU", "SAN")

	// Assert
	if err != nil {
		t.Error(err)
	}
	if minimalDistance != 301 {
		t.Error("Wrong minimal distance", minimalDistance)
	}
}
