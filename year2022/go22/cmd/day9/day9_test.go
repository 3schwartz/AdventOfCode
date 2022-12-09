package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_findTailVisited(t *testing.T) {
	// Arrange
	input := readData(9)

	// Act
	tailVisitedCount := findTailVisitedCount(input)

	// Assert
	if diff := cmp.Diff(tailVisitedCount, 13); diff != "" {
		t.Error(diff)
	}
}

func readData(day int) string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day%d_test_data.txt", day))
	if err != nil {
		panic(err)
	}
	return string(f)
}
