package main

import (
	"fmt"
	"math"
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
	if diff := cmp.Diff(tailVisitedCount.size(), 1); diff != "" {
		t.Error(diff)
	}
}

func Test_findTailVisitedLargeRobe(t *testing.T) {
	// Arrange
	input := readData("2")

	// Act
	tailVisitedCount := findTailVisitedLargeRobe(input)
	tailVisitedCount.print()

	// Assert
	if diff := cmp.Diff(tailVisitedCount.size(), 36); diff != "" {
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

func (t tailVisited) print() {
	rotated := map[coord2d]struct{}{}
	for key := range t {
		rotated[coord2d{-key.y, key.x}] = struct{}{}

	}
	minX, minY := math.MaxInt, math.MaxInt
	maxX, maxY := math.MinInt, math.MinInt
	for c := range rotated {
		if c.x > maxX {
			maxX = c.x
		}
		if c.x < minX {
			minX = c.x
		}
		if c.y > maxY {
			maxY = c.y
		}
		if c.y < minY {
			minY = c.y
		}
	}
	for x := minX; x <= maxX; x++ {
		for y := minY; y <= maxY; y++ {
			_, ok := rotated[coord2d{x, y}]
			if ok {
				fmt.Print("#")
				continue
			}
			fmt.Print(".")
		}
		fmt.Print("\n")
	}
}

func readData(suffix string) string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day9_test%s_data.txt", suffix))
	if err != nil {
		panic(err)
	}
	return string(f)
}
