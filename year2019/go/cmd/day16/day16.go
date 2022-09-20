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

	// firstEight := cleanSignal(input, 100, 1, 0)

	// fmt.Printf("Part 1: %s\n", firstEight)

	offset := getOffsetFromBeginning(input)

	eightByOffset := cleanSignal(input, 100, 10_000, offset)

	fmt.Printf("Part 2: %s\n", eightByOffset)
}

func getOffsetFromBeginning(input string) int {
	offset, err := strconv.Atoi(input[:7])
	if err != nil {
		panic(err)
	}
	return offset
}

func readData() string {
	f, err := os.ReadFile("../../../data/day16_data.txt")
	if err != nil {
		panic(err)
	}
	input := string(f)
	return input
}

func cleanSignal(input string, phaseCount int, inputRepeated int, offset int) string {
	inputLength := len(input)
	totalRowLength := inputLength * inputRepeated

	output := make([]int, totalRowLength)
	for j := 0; j < inputRepeated; j++ {
		for i, v := range input {
			output[i+(j*inputLength)] = int(v - '0')
		}
	}

	for i := 0; i < phaseCount; i++ {
		newOutput := make([]int, totalRowLength)
		for row := 0; row < totalRowLength; row++ {
			var currentOutput int
			for idx := 0; idx < totalRowLength; idx++ {
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
	firstEight := outputAfterPhase[offset:(8 + offset)]

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
