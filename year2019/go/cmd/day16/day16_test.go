package main

import (
	"strconv"
	"testing"
)

func Test_givenInputSignalThenCorrectOutput(t *testing.T) {
	// Arrange
	input := 12345678
	inputAsString := strconv.Itoa(input)
	inputLength := len(strconv.Itoa(input))

	// Act
	output := make([]int, inputLength)
	for i, v := range inputAsString {
		output[i] = int(v - '0')
	}

	newOutput := make([]int, inputLength)
	for row := 0; row < inputLength; row++ {
		var currentOutput int
		for idx := 0; idx < inputLength; idx++ {
			toApply := findPatternToApply(idx, row, pattern)
			currentOutput += toApply * output[idx]
		}
		if currentOutput < 0 {
			currentOutput *= -1
		}
		toInsert := currentOutput % 10
		newOutput[row] = toInsert
	}

	outputAfterPhase := toOneInt(newOutput)
	lastEight := outputAfterPhase % 100_000_000

	// Assert
	if lastEight != 48226158 {
		t.Errorf("wrong output: %d", lastEight)
	}

}

func Test_givenPhaseCountWhenGivenInputSignalThenCorrectOutput(t *testing.T) {
	data := []struct {
		name       string
		phaseCount int
		input      string
		expected   int
	}{
		{"1", 1, "12345678", 48226158},
		{"2", 2, "12345678", 34040438},
		{"3", 3, "12345678", 3415518},
		{"4", 4, "12345678", 1029498},
		{"a", 100, "80871224585914546619083218645595", 24176176},
		{"b", 100, "19617804207202209144916044189917", 73745418},
		{"c", 100, "69317163492948606335995924319873", 52432133},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			lastEight := cleanSignal(d.input, d.phaseCount)

			// Assert
			if lastEight != d.expected {
				t.Errorf("wrong output: %d", lastEight)
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
