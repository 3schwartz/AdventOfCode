package main

import (
	"advent/pkg/coders"
	"os"
	"strings"
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
			13087969,
		},
		{
			"Part2",
			5,
			14110739,
		},
	}
	// Arrange
	codes := readData()
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

func readData() []string {
	f, err := os.ReadFile("../../../data/day5_data.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), ",")
}
