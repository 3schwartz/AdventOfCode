package main

import (
	"advent2022/pkg/io"
	"testing"
)

func Test_integerToSnafu(t *testing.T) {
	// Arrange
	data := []struct {
		integer int
		snafu   string
	}{
		{1, "1"},
		{2, "2"},
		{3, "1="},
		{4, "1-"},
		{5, "10"},
		{6, "11"},
		{7, "12"},
		{8, "2="},
		{9, "2-"},
		{10, "20"},
		{15, "1=0"},
		{20, "1-0"},
		{2022, "1=11-2"},
		{12345, "1-0---0"},
		{314159265, "1121-1110-1=0"},
	}
	for _, d := range data {
		t.Run(d.snafu, func(t *testing.T) {
			// Act
			result := integerToSnafu(d.integer)

			// Assert
			if result != d.snafu {
				t.Error(result)
			}
		})
	}
}

func Test_snafuToInteger(t *testing.T) {
	// Arrange
	data := []struct {
		snafu   string
		integer int
	}{
		{"2=-01", 976},
		{"12111", 906},
		{"2=0=", 198},
		{"21", 11},
		{"2=01", 201},
		{"111", 31},
		{"20012", 1257},
		{"112", 32},
		{"1=-1=", 353},
		{"1-12", 107},
		{"12", 7},
		{"1=", 3},
		{"122", 37},
	}
	for _, d := range data {
		t.Run(d.snafu, func(t *testing.T) {
			// Act
			result := snafuToInteger(d.snafu)

			// Assert
			if result != d.integer {
				t.Error(result)
			}
		})
	}
}

func Test_findSnafuSum(t *testing.T) {
	input := io.ReadData("25_test")

	// Act
	sum := findSnafuSum(input)

	// Assert
	if sum != 4890 {
		t.Error(sum)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("25_test")

	// Act
	sum := findSnafuSum(input)
	snafu := integerToSnafu(sum)

	// Assert
	if snafu != "2=-1=0" {
		t.Error(snafu)
	}
}
