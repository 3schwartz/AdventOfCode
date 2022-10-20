package main

import (
	"fmt"
	"testing"
)

func equals(actual []int, expected []int) bool {
	ok := true
	for i := 0; i < len(expected); i++ {
		ok = ok && expected[i] == actual[i]
	}
	return ok
}

func Test_testData(t *testing.T) {
	// Arrange
	lines := parseData("day22_test1_data")
	expected := [10]int{9, 2, 5, 8, 1, 4, 7, 0, 3, 6}

	// Act
	actual := iterateLines(lines, 10)

	// Assert
	ok := equals(actual, expected[:])
	if !ok {
		t.Error(actual)
	}
}

func Test_dealStack(t *testing.T) {
	// Arrange
	input := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	expected := [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}

	// Act
	actual := stack(input[:])

	// Assert
	ok := equals(actual, expected[:])
	if !ok {
		t.Error(actual)
	}
}

func Test_cutCards(t *testing.T) {
	// Arrange
	data := []struct {
		cut      int
		input    [10]int
		expected [10]int
	}{
		{
			cut:      3,
			input:    [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9},
			expected: [10]int{3, 4, 5, 6, 7, 8, 9, 0, 1, 2},
		},
		{
			cut:      -4,
			input:    [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9},
			expected: [10]int{6, 7, 8, 9, 0, 1, 2, 3, 4, 5},
		},
		{
			cut:      -4,
			input:    [10]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0},
			expected: [10]int{3, 2, 1, 0, 9, 8, 7, 6, 5, 4},
		},
	}
	for _, d := range data {
		t.Run(fmt.Sprintf("Cut: %d", d.cut), func(t *testing.T) {
			// Act
			actual := cut(d.input[:], d.cut)

			// Assert
			ok := equals(actual, d.expected[:])
			if !ok {
				t.Error(actual)
			}
		})
	}
}

func Test_dealIncrement(t *testing.T) {
	// Arrange
	input := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	expected := [10]int{0, 7, 4, 1, 8, 5, 2, 9, 6, 3}

	// Act
	actual := increment(input[:], 3)

	// Assert
	ok := equals(actual, expected[:])
	if !ok {
		t.Error(actual)
	}
}
