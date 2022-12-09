package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_visibleTreesCorrect(t *testing.T) {
	// Arrange
	input := readData(8)

	// Act
	scannedTreeGrid := createTreeGrid(input)
	visibleTrees := scannedTreeGrid.getVisibleTrees()

	// Assert
	if diff := cmp.Diff(visibleTrees, 21); diff != "" {
		t.Error(diff)
	}
}

func Test_treeCoverCorrect(t *testing.T) {
	// Arrange
	input := readData(8)

	// Act
	scannedTreeGrid := createTreeGrid(input)
	coverScore := scannedTreeGrid.getTreeCoverScore()

	// Assert
	if diff := cmp.Diff(coverScore, 8); diff != "" {
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
