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

func stack(input []int) []int {
	output := make([]int, len(input))
	shift := len(input) - 1
	for i := 0; i < len(input); i++ {
		output[i] = -1*input[i] + shift
	}
	return output
}

func Test_cutCards(t *testing.T) {
	// Arrange
	data := []struct {
		cut      int
		expected [10]int
	}{
		{
			cut:      3,
			expected: [10]int{3, 4, 5, 6, 7, 8, 9, 0, 1, 2},
		},
		{
			cut:      -4,
			expected: [10]int{6, 7, 8, 9, 0, 1, 2, 3, 4, 5},
		},
	}
	input := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	for _, d := range data {
		t.Run(fmt.Sprintf("Cut: %d", d.cut), func(t *testing.T) {
			// Act
			actual := cut(input[:], d.cut)

			// Assert
			ok := equals(actual, d.expected[:])
			if !ok {
				t.Error(actual)
			}
		})
	}
}

func cut(input []int, cut int) []int {
	output := make([]int, len(input))
	length := len(input)
	shift := cut
	if cut < 0 {
		shift += length
	}
	modulo := length
	for i := 0; i < len(input); i++ {
		output[i] = (input[i] + shift) % modulo
	}
	return output
}

func Test_dealIncrement(t *testing.T) {
	// Arrange
	input := [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	expected := [10]int{0, 7, 4, 1, 8, 5, 2, 9, 6, 3}
	// {0,-6,-2,2,-4,0,4,-2,2,6}

	// Act
	actual := increment(input[:], 3)

	// Assert
	ok := equals(actual, expected[:])
	if !ok {
		t.Error(actual)
	}
}

func increment(input []int, inc int) []int {
	output := make([]int, len(input))
	shift := len(input)
	// multiplier := inc
	modulo := inc
	for i := 0; i < len(input); i++ {
		output[i] = input[i] + shift%modulo
	}
	return output
}
