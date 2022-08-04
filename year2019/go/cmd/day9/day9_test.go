package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_solution(t *testing.T) {
	data := []struct {
		name     string
		input    int
		expected int
	}{
		{
			"Part1",
			1,
			3460311188,
		},
		{
			"Part2",
			2,
			42202,
		},
	}
	// Arrange
	codes := read.ReadData("day9")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.IntCoder{}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {

			// Act
			actual := intCoder.RunWithInput(intCodes, d.input)

			// Assert
			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}
}
