package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_findSmallest(t *testing.T) {
	// Arrange
	input := readData(7)
	root := createGraph(input)
	_ = root.getSum()
	fileSystemSize := 70_000_000
	freeSize := 30_000_000

	// Act
	size := root.findSizeOffSmallestDirectoryWhichFreesEnough(fileSystemSize, freeSize)

	// Assert
	if diff := cmp.Diff(size, 24933642); diff != "" {
		t.Error(diff)
	}
}

func Test_whenFindSum_thenCorrect(t *testing.T) {
	// Arrange
	input := readData(7)
	root := createGraph(input)

	// Act
	sum := root.findSumCountBelow(100_000)

	// Assert
	if diff := cmp.Diff(sum, 95_437); diff != "" {
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
