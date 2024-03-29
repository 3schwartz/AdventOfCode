package main

import (
	"testing"
)

func Test_givenOffsetFromBeginningWhenGivenInputSignalThenCorrectOutput(t *testing.T) {
	data := []struct {
		name       string
		phaseCount int
		input      string
		expected   string
	}{
		{"a", 100, "03036732577212944063491565474664", "84462026"},
		{"b", 100, "02935109699940807407585447034323", "78725270"},
		{"c", 100, "03081770884921959731165446850517", "53553731"},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			firstEightAfterOffset := cleanSignalByOffset(d.input, 100)

			// Assert
			if firstEightAfterOffset != d.expected {
				t.Errorf("wrong output: %s", firstEightAfterOffset)
			}
		})
	}
}

func Test_givenPhaseCountWhenGivenInputSignalThenCorrectOutput(t *testing.T) {
	data := []struct {
		name       string
		phaseCount int
		input      string
		expected   string
	}{
		{"1", 1, "12345678", "48226158"},
		{"2", 2, "12345678", "34040438"},
		{"3", 3, "12345678", "03415518"},
		{"4", 4, "12345678", "01029498"},
		{"a", 100, "80871224585914546619083218645595", "24176176"},
		{"b", 100, "19617804207202209144916044189917", "73745418"},
		{"c", 100, "69317163492948606335995924319873", "52432133"},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			firstEight := cleanSignal(d.input, d.phaseCount)

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
