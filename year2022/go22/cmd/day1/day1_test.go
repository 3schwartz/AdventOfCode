package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_readFile(t *testing.T) {
	// Arrange
	expected := "Hello Advent"

	// Act
	actual := readData()

	// Assert
	if diff := cmp.Diff(expected, actual); diff != "" {
		t.Error(diff)
	}
}
