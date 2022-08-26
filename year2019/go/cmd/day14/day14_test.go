package main

import (
	"encoding/json"
	"testing"
)

func Test_correct_read(t *testing.T) {
	// Arrange & Act
	chemicals := parseChemicals("day14_test")

	// Assert
	if len(*chemicals) != 6 {
		s, _ := json.MarshalIndent(chemicals, "", "\t")
		t.Errorf("chemicals not constructed correct: %s", string(s))
	}
}

func Test_correct_raw_count(t *testing.T) {
	data := []struct {
		fileName string
		expected int
	}{
		{"day14_test", 31},
		{"day14_test2", 165},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			chemicals := parseChemicals(d.fileName)

			// Act
			rawCount, _ := chemicals.getRawCountFrom("FUEL", 1, map[string]int{})

			// Assert
			if rawCount != d.expected {
				t.Errorf("%s: expected: %d, got: %d", d.fileName, d.expected, rawCount)
			}
		})
	}
}
