package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part1Small(t *testing.T) {
	// Arrange
	input := io.ReadData("23_test1")

	// Act
	overview := createOverview(input)
	elves := doRounds(overview, 1)
	empty := elves.findEmpty()

	// Assert
	if empty != 5 {
		t.Error(empty)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("23_test")

	// Act
	overview := createOverview(input)
	elves := doRounds(overview, 10)
	empty := elves.findEmpty()

	// Assert
	if empty != 110 {
		t.Error(empty)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("23_test")

	// Act
	overview := createOverview(input)
	rounds := doToZero(overview)

	// Assert
	if rounds != 20 {
		t.Error(rounds)
	}
}
