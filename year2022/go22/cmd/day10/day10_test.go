package main

import (
	"advent2022/pkg/io"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_whenRunCycle_ThenCorrectSignalStrength(t *testing.T) {
	// Arrange
	input := io.ReadData("10_test")

	// Act
	signalStrength := findSignalStrength(input)

	// Assert
	if diff := cmp.Diff(signalStrength, 13140); diff != "" {
		t.Error(diff)
	}
}
