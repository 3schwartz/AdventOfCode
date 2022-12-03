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

	simpleSum := 0
	complexSum := 0
	for _, round := range rounds {
		simpleGame := createRound(round, Simple)
		simpleChoiceScore := simpleGame.getChoiceScore()
		simpleWinScore := simpleGame.getWinScore()
		simpleSum += simpleChoiceScore + simpleWinScore

		complexGame := createRound(round, Complex)
		complexChoiceScore := complexGame.getChoiceScore()
		complexWinScore := complexGame.getWinScore()
		complexSum += complexChoiceScore + complexWinScore

	}
	fmt.Printf("Part 1: %d\n", simpleSum)
	fmt.Printf("Part 2: %d\n", complexSum)
}

type GameType int

const (
	Simple GameType = iota
	Complex
)

type game interface {
	getChoiceScore() int
	getWinScore() int
}

func createRound(input string, gameType GameType) game {
	cards := strings.Split(input, " ")
	p1 := symbolLookup(cards[0])
	p2 := symbolLookup(cards[1])
	switch gameType {
	case Complex:
		return gameComplexRound{
			p1: p1,
			p2: p2,
		}
	default:
		return gameRound{
			p1: p1,
			p2: p2,
		}
	}
}

func symbolLookup(symbol string) int {
	switch symbol {
	case "A", "X":
		return 1
	case "B", "Y":
		return 2
	default: // C || "Z"
		return 3
	}
}

// X: loose
// Y: draw
// Z: win
type gameComplexRound struct {
	p1 int
	p2 int
}

func (g gameComplexRound) getChoiceScore() int {
	// Draw
	if g.p2 == 2 {
		return g.p1
	}
	// Loose
	if g.p2 == 1 {
		return (g.p1+1)%3 + 1
	}
	// Win
	return g.p1%3 + 1
}

func (g gameComplexRound) getWinScore() int {
	return (g.p2 - 1) * 3
}

type gameRound struct {
	p1 int
	p2 int
}

func (g gameRound) getWinScore() int {
	if g.p1 == g.p2 {
		return 3
	}
	if g.p1%3+1 == g.p2 {
		return 6
	}
	return 0
}

func (g gameRound) getChoiceScore() int {
	return g.p2
}

func readData() string {
	f, err := os.ReadFile("../../../data/day2_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
