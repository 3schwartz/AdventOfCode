package main

import (
	"advent/pkg/coders"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_Part1(t *testing.T) {
	// Arrange
	codes := readData()
	intCodes := coders.ParseIntCodes(codes)
	intCoderInstance := coders.IntCoder{}

	// Act
	actual := intCoderInstance.RunWithNounAndVerb(intCodes, 12, 2)

	// Assert
	if diff := cmp.Diff(5305097, actual[0]); diff != "" {
		t.Error(diff)
	}
}

func Test_Part2(t *testing.T) {
	// Arrange
	codes := readData()
	intCodes := coders.ParseIntCodes(codes)
	intCoderInstance := coders.IntCoder{}

	// Act
	actual, err := intCoderInstance.FindOptimalOutput(intCodes, 19690720)

	// Assert
	if err != nil {
		t.Error(err)
	}
	if diff := cmp.Diff(4925, actual); diff != "" {
		t.Error(diff)
	}
}
