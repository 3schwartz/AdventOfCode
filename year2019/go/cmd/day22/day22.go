package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	lines := parseData("day22_data")
	shuffler := deckShuffler{}

	output := shuffler.iterateLines(lines, 10007)

	fmt.Printf("Part 1: %d", shuffler.findIndex(output, 2019))
}

func parseData(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s.txt", fileName))
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(f), "\r\n")

	return lines
}

type deckShuffler struct{}

func (d deckShuffler) findIndex(deck []int, valueAtIndex int) int {
	for i, card := range deck {
		if card == valueAtIndex {
			return i
		}
	}
	return 0
}

func (d deckShuffler) iterateLines(lines []string, size int) []int {
	cutSize := len("cut ")
	incSize := len("deal with increment ")
	input := d.initializeArray(size)

	for _, line := range lines {
		if strings.Contains(line, "cut") {
			cutInput, err := strconv.Atoi(line[cutSize:])
			if err != nil {
				panic(err)
			}
			input = d.cut(input, cutInput)
			continue
		}
		if strings.Contains(line, "deal with increment") {
			incInput, err := strconv.Atoi(line[incSize:])
			if err != nil {
				panic(err)
			}
			input = d.increment(input, incInput)
			continue
		}
		if strings.Contains(line, "deal into new stack") {
			input = d.stack(input)
			continue
		}
		panic(line)
	}
	return input
}

func (s deckShuffler) stack(input []int) []int {
	output := make([]int, len(input))
	shift := len(input) - 1
	for i := 0; i < len(input); i++ {
		idx := -1*i + shift
		output[idx] = input[i]
	}
	return output
}

func (s deckShuffler) increment(input []int, inc int) []int {
	length := len(input)
	output := make([]int, length)
	idx := 0
	for i := 0; i < length; i++ {
		output[idx] = input[i]
		idx = modulo(idx+inc, length)
	}
	return output
}

func (s deckShuffler) cut(input []int, cut int) []int {
	length := len(input)
	output := make([]int, length)
	shift := cut
	for i := 0; i < length; i++ {
		idx := modulo(i-shift, length)
		output[idx] = input[i]
	}
	return output
}

func (s deckShuffler) initializeArray(size int) []int {
	input := make([]int, size)
	for i := 0; i < size; i++ {
		input[i] = i
	}
	return input
}

func modulo(in int, mod int) int {
	return (in%mod + mod) % mod
}
