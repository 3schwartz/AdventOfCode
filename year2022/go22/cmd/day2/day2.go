package main

// A, X: Rock
// B, Y: Paper
// C, Z: Scissors

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	input := readData()
	rounds := strings.Split(input, "\r\n")

	totalSum := 0
	for _, round := range rounds {
		game := createGameRound(round)
		choiceScore := game.getChoiceScore()
		winScore := game.getWinScore()
		totalSum += choiceScore + winScore
	}
	fmt.Printf("Part 1: %d\n", totalSum)

	totalSumComplex := 0
	for _, round := range rounds {
		game := createGameComplexRound(round)
		choiceScore := game.getChoiceScore()
		winScore := game.getWinScore()
		totalSumComplex += choiceScore + winScore
	}
	fmt.Printf("Part 2: %d\n", totalSumComplex)
}

// X: loose
// Y: draw
// Z: win
type gameComplexRound struct {
	p1 string
	p2 string
}

func createGameComplexRound(input string) gameComplexRound {
	cards := strings.Split(input, " ")
	return gameComplexRound{
		p1: cards[0],
		p2: cards[1],
	}
}

func (g gameComplexRound) getChoiceScore() int {
	// Draw
	if g.p2 == "Y" {
		return g.getChoiceScoreFromPlayer(g.p1)
	}
	// Loose
	if g.p2 == "X" {
		switch g.p1 {
		case "A":
			return 3
		case "B":
			return 1
		case "C":
			return 2
		}
	}
	// Win
	switch g.p1 {
	case "A":
		return 2
	case "B":
		return 3
	default: // case "C":
		return 1
	}
}

func (g gameComplexRound) getWinScore() int {
	switch g.p2 {
	case "X":
		return 0
	case "Y":
		return 3
	default: //case "Z":
		return 6
	}
}

func (g gameComplexRound) getChoiceScoreFromPlayer(choice string) int {
	switch choice {
	case "A":
		return 1
	case "B":
		return 2
	default: //case "C":
		return 3
	}
}

type gameRound struct {
	p1 string
	p2 string
}

func (g gameRound) getWinScore() int {
	if g.p1 == "A" && g.p2 == "X" || g.p1 == "B" && g.p2 == "Y" || g.p1 == "C" && g.p2 == "Z" {
		return 3
	}
	if g.p1 == "A" && g.p2 == "Y" || g.p1 == "B" && g.p2 == "Z" || g.p1 == "C" && g.p2 == "X" {
		return 6
	}
	return 0
}

func (g gameRound) getChoiceScore() int {
	switch g.p2 {
	case "X":
		return 1
	case "Y":
		return 2
	default: //case "Z":
		return 3
	}
}

func createGameRound(input string) gameRound {
	cards := strings.Split(input, " ")
	return gameRound{
		p1: cards[0],
		p2: cards[1],
	}
}

func readData() string {
	path, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	fmt.Println(path)
	f, err := os.ReadFile("../../../data/day2_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
