package coders

import (
	"fmt"
	"strconv"
)

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

type IntCoder struct {
}

func (ic IntCoder) RunInstructions(codesInput []int, noun int, verb int) []int {
	codes := make([]int, len(codesInput))
	copy(codes, codesInput)
	ic.runModifyingInstructions(codes, noun, verb)
	return codes
}

// Modifies given slice
func (ic IntCoder) runModifyingInstructions(codes []int, noun int, verb int) {
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

func (ic IntCoder) FindOptimalOutput(codesInput []int, optimalValue int) (int, error) {
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
