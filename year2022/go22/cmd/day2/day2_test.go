package main

import (
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := readData()
	rounds := strings.Split(input, "\r\n")

	// Act
	totalSum := 0
	for _, round := range rounds {
		game := createGameRound(round)
		choiceScore := game.getChoiceScore()
		winScore := game.getWinScore()
		totalSum += choiceScore + winScore
	}

	// Assert
	if diff := cmp.Diff(totalSum, 11666); diff != "" {
		t.Error(diff)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := readData()
	rounds := strings.Split(input, "\r\n")

	// Act
	totalSumComplex := 0
	for _, round := range rounds {
		game := createGameComplexRound(round)
		choiceScore := game.getChoiceScore()
		winScore := game.getWinScore()
		totalSumComplex += choiceScore + winScore
	}

	// Assert
	if diff := cmp.Diff(totalSumComplex, 12767); diff != "" {
		t.Error(diff)
	}
}
