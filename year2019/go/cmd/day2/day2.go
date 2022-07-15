package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	codes := readData()
	intCodes := ParseIntCodes(codes)
	intCoderInstance := intCoder{}
	codesModified := intCoderInstance.runInstructions(intCodes, 12, 2)
	fmt.Printf("Part 1: %d\n", codesModified[0])

	optima, err := intCoderInstance.findOptimalOutput(intCodes, 19690720)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 2: %d\n", optima)
}

type intCoder struct {
}

func (ic intCoder) runInstructions(codesInput []int, noun int, verb int) []int {
	codes := make([]int, len(codesInput))
	copy(codes, codesInput)
	ic.runModifyingInstructions(codes, noun, verb)
	return codes
}

// Modifies given slice
func (ic intCoder) runModifyingInstructions(codes []int, noun int, verb int) {
	codes[1] = noun
	codes[2] = verb
	var idx int
optLoop:
	for {

		switch optCode := codes[idx]; optCode {
		case 1:
			codes[codes[idx+3]] = codes[codes[idx+2]] + codes[codes[idx+1]]
		case 2:
			codes[codes[idx+3]] = codes[codes[idx+2]] * codes[codes[idx+1]]
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode now know: %d", optCode))
		}
		idx += 4
	}
}

func (ic intCoder) findOptimalOutput(codesInput []int, optimalValue int) (int, error) {
	codes := make([]int, len(codesInput))

	for i := 0; i < 100; i++ {
		for j := 0; j < 100; j++ {
			copy(codes, codesInput)
			ic.runModifyingInstructions(codes, i, j)
			if codes[0] == optimalValue {
				return 100*i + j, nil
			}
		}
	}
	return 0, fmt.Errorf("no noun and verb found for optimal value: %d", optimalValue)
}

func readData() []string {
	f, err := os.ReadFile("../../../data/day2_data.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), ",")
}

func ParseIntCodes(codes []string) []int {
	intCodes := make([]int, len(codes))
	for i, code := range codes {
		intCode, err := strconv.Atoi(code)
		if err != nil {
			panic(err)
		}
		intCodes[i] = intCode
	}
	return intCodes
}
