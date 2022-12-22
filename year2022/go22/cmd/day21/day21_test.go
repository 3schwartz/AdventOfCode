package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("21_test")
	monkeys := createMonkeyTree(input)

	// Act
	sum := monkeys.findCorrectInitial("root", "humn")

	// Assert
	if sum != 301 {
		t.Error(sum)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("21_test")
	monkeys := createMonkeyTree(input)

	// Act
	sum := monkeys.findSumFrom("root")

	// Assert
	if sum != 152 {
		t.Error(sum)
	}
}
