package coders

import (
	"fmt"
	"math"
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

func (ic IntCoder) RunWithNounAndVerb(codesInput []int, noun int, verb int) []int {
	codes := make([]int, len(codesInput))
	copy(codes, codesInput)
	ic.runModifyingInstructions(codes, noun, verb)
	return codes
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

func (ic IntCoder) RunWithInput(codesInput []int, input int) int {
	codes := make([]int, len(codesInput))
	copy(codes, codesInput)
	lastOutput := ic.runModifyingInstructionsNew(codes, input)
	return lastOutput
}

// Modifies given slice
func (ic IntCoder) runModifyingInstructionsNew(codes []int, input int) int {
	var idx int
	outputs := make([]int, 0)
optLoop:
	for {
		execution := codes[idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[ic.getIdxFromMode(codes, execution, 3, idx)] =
				codes[ic.getIdxFromMode(codes, execution, 2, idx)] + codes[ic.getIdxFromMode(codes, execution, 1, idx)]
			idx += 4
		case 2:
			codes[ic.getIdxFromMode(codes, execution, 3, idx)] =
				codes[ic.getIdxFromMode(codes, execution, 2, idx)] * codes[ic.getIdxFromMode(codes, execution, 1, idx)]
			idx += 4
		case 3:
			codes[ic.getIdxFromMode(codes, execution, 1, idx)] = input
			idx += 2
		case 4:
			output := codes[ic.getIdxFromMode(codes, execution, 1, idx)]
			outputs = append(outputs, output)
			idx += 2
		case 5:
			if codes[ic.getIdxFromMode(codes, execution, 1, idx)] != 0 {
				idx = codes[ic.getIdxFromMode(codes, execution, 2, idx)]
				break
			}
			idx += 3
		case 6:
			if codes[ic.getIdxFromMode(codes, execution, 1, idx)] == 0 {
				idx = codes[ic.getIdxFromMode(codes, execution, 2, idx)]
				break
			}
			idx += 3
		case 7:
			var toAssign int
			if codes[ic.getIdxFromMode(codes, execution, 1, idx)] < codes[ic.getIdxFromMode(codes, execution, 2, idx)] {
				toAssign = 1
			}
			codes[ic.getIdxFromMode(codes, execution, 3, idx)] = toAssign
			idx += 4
		case 8:
			var toAssign int
			if codes[ic.getIdxFromMode(codes, execution, 1, idx)] == codes[ic.getIdxFromMode(codes, execution, 2, idx)] {
				toAssign = 1
			}
			codes[ic.getIdxFromMode(codes, execution, 3, idx)] = toAssign
			idx += 4
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}

	return outputs[len(outputs)-1]
}

// Modifies given slice
// Fine up until day 2
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

func (cc IntCoder) getIdxFromMode(codes []int, execution int, parameterPosition int, idx int) int {
	mode := execution / int(math.Pow(10, 1+float64(parameterPosition)))
	mode %= 10
	if mode == 1 {
		return idx + parameterPosition
	}
	return codes[idx+parameterPosition]
}
