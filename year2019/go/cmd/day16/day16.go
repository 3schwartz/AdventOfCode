package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

var pattern [4]int

func init() {
	pattern = [4]int{0, 1, 0, -1}
}

func main() {
	input := readData()
	lastEight := cleanSignal(input, 100)

	fmt.Printf("Part 1: %d\n", lastEight)
}

func readData() string {
	f, err := os.ReadFile("../../../data/day16_data.txt")
	if err != nil {
		panic(err)
	}
	input := string(f)
	return input
}

func cleanSignal(input string, phaseCount int) int {
	inputLength := len(input)

	// Act
	output := make([]int, inputLength)
	for i, v := range input {
		output[i] = int(v - '0')
	}
	for i := 0; i < phaseCount; i++ {
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
		output = newOutput
	}

	outputAfterPhase := toOneInt(output)
	lastEight := outputAfterPhase % 100_000_000
	return lastEight
}

func toOneInt(output []int) int {
	stringOutput := make([]string, len(output))
	for i, out := range output {
		stringOutput[i] = strconv.Itoa(out)
	}
	oneString := strings.Join(stringOutput, "")
	outOneNumber, err := strconv.Atoi(oneString)
	if err != nil {
		panic(err)
	}
	return outOneNumber
}

// The idx and row is indexed from 0.
// Skip offset of the whole pattern left by one.
func findPatternToApply(idx int, row int, pattern [4]int) int {
	idx++
	row++
	toApply := pattern[(idx/row)%len(pattern)]
	return toApply
}
