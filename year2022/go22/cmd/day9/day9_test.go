package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_findTailVisitedLargeRobeSimple(t *testing.T) {
	// Arrange
	input := readData("")

	// Act
	tailVisitedCount := findTailVisitedLargeRobe(input)

	// Assert
	if diff := cmp.Diff(tailVisitedCount, 1); diff != "" {
		t.Error(diff)
	}
}

func Test_findTailVisitedLargeRobe(t *testing.T) {
	// Arrange
	input := readData("2")

	// Act
	tailVisitedCount := findTailVisitedLargeRobe(input)

	// Assert
	if diff := cmp.Diff(tailVisitedCount, 36); diff != "" {
		t.Error(diff)
	}
}

func Test_findTailVisited(t *testing.T) {
	// Arrange
	input := readData("")

	// Act
	tailVisitedCount := findTailVisitedCount(input)

	// Assert
	if diff := cmp.Diff(tailVisitedCount, 13); diff != "" {
		t.Error(diff)
	}
}

func readData(suffix string) string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day9_test%s_data.txt", suffix))
	if err != nil {
		panic(err)
	}
	return string(f)
}
