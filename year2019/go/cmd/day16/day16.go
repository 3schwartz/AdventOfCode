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
	firstEight := cleanSignal(input, 100)

	fmt.Printf("Part 1: %s\n", firstEight)
}

func readData() string {
	f, err := os.ReadFile("../../../data/day16_data.txt")
	if err != nil {
		panic(err)
	}
	input := string(f)
	return input
}

func cleanSignal(input string, phaseCount int) string {
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

	outputAfterPhase := toOneString(output)
	firstEight := outputAfterPhase[:8]

	return firstEight
}

func toOneString(output []int) string {
	stringOutput := make([]string, len(output))
	for i, out := range output {
		stringOutput[i] = strconv.Itoa(out)
	}
	oneString := strings.Join(stringOutput, "")
	return oneString
}

// The idx and row is indexed from 0.
// Skip offset of the whole pattern left by one.
func findPatternToApply(idx int, row int, pattern [4]int) int {
	idx++
	row++
	toApply := pattern[(idx/row)%len(pattern)]
	return toApply
}
