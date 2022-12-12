package main

import (
	"advent2022/pkg/io"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("12_test")

	// Act
	actual := multipleShortestPath(input)

	// Assert
	if diff := cmp.Diff(actual, 29); diff != "" {
		t.Error(diff)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("12_test")

	// Act
	actual := singleShortestPath(input)

	// Assert
	if diff := cmp.Diff(actual, 31); diff != "" {
		t.Error(diff)
	}
}
