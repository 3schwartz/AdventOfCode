package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_getFuel(t *testing.T) {
	// Arrange
	expected := 2

	// Act
	actual := getFuel(12)

	// Assert
	if diff := cmp.Diff(expected, actual); diff != "" {
		t.Error(diff)
	}
}
