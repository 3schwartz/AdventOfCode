package main

import (
	"testing"
)

func Test_givenPhaseCountWhenGivenInputSignalThenCorrectOutput(t *testing.T) {
	data := []struct {
		name          string
		phaseCount    int
		inputRepeated int
		offset        int
		input         string
		expected      string
	}{
		// {"1", 1, 1, 0, "12345678", "48226158"},
		// {"2", 2, 1, 0, "12345678", "34040438"},
		// {"3", 3, 1, 0, "12345678", "03415518"},
		// {"4", 4, 1, 0, "12345678", "01029498"},
		// {"a", 100, 1, 0, "80871224585914546619083218645595", "24176176"},
		// {"b", 100, 1, 0, "19617804207202209144916044189917", "73745418"},
		// {"c", 100, 1, 0, "69317163492948606335995924319873", "52432133"},
		{"repeat_a", 100, 10_000, 303673, "03036732577212944063491565474664", "84462026"},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			firstEight := cleanSignal(d.input, d.phaseCount, d.inputRepeated, d.offset)

			// Assert
			if firstEight != d.expected {
				t.Errorf("wrong output: %s", firstEight)
			}
		})
	}
}

func Test_givenPatternWhenGiveRowCountThenCorrectPatternIndex(t *testing.T) {
	data := []struct {
		name                   string
		idx                    int
		row                    int
		expectedPatternToApply int
	}{
		{"row0", 0, 0, 1},
		{"row0", 2, 0, -1},
		{"row0", 6, 0, -1},
		{"row0", 7, 0, 0},
		{"row1", 0, 1, 0},
		{"row1", 1, 1, 1},
		{"row1", 2, 1, 1},
		{"row1", 3, 1, 0},
		{"row1", 6, 1, -1},
		{"row1", 7, 1, 0},
		{"row7", 6, 7, 0},
		{"row7", 7, 7, 1},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Arrange
			// pattern := [4]int{0, 1, 0, -1}

			// Act
			patternToApply := findPatternToApply(d.idx, d.row, pattern)

			// Assert
			if patternToApply != d.expectedPatternToApply {
				t.Errorf("wrong pattern to apply: %d", patternToApply)
			}
		})
	}

}
