package main

import (
	"fmt"
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_modulo(t *testing.T) {
	// Arrange
	data := []struct {
		input    int
		expected int
	}{
		{1, 3},
		{2, 1},
		{3, 2},
	}
	for _, d := range data {
		t.Run(fmt.Sprintf("%d", d.input), func(t *testing.T) {
			// Act
			actual := modulo(d.input+1, 3) + 1
			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}

}

func Test_part1(t *testing.T) {
	// Arrange
	input := readData()
	rounds := strings.Split(input, "\r\n")

	// Act
	totalSum := 0
	for _, round := range rounds {
		game := createRound(round, Simple)
		choiceScore := game.getChoiceScore()
		winScore := game.getWinScore()
		totalSum += choiceScore + winScore
	}

	// Assert
	if diff := cmp.Diff(totalSum, 11666); diff != "" {
		t.Error(diff)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := readData()
	rounds := strings.Split(input, "\r\n")

	// Act
	totalSumComplex := 0
	for _, round := range rounds {
		game := createRound(round, Complex)
		choiceScore := game.getChoiceScore()
		winScore := game.getWinScore()
		totalSumComplex += choiceScore + winScore
	}

	// Assert
	if diff := cmp.Diff(totalSumComplex, 12767); diff != "" {
		t.Error(diff)
	}
}
