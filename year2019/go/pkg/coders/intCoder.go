package coders

import (
	"fmt"
	"math"
	"strconv"
)

type Coordinate struct {
	x int
	y int
}

func (c Coordinate) String() string {
	return fmt.Sprintf("X: %d, Y: %d", c.x, c.y)
}

func (c Coordinate) IsEmpty() bool {
	return c.x == 0 && c.y == 0
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

type IntCoder struct {
	idx          int
	relativeBase int
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

func (ic *IntCoder) GenerateCodes(codesInput []int) map[int]int {
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	return codes
}

func (ic *IntCoder) RunWithInput(codesInput []int, input int) int {
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	lastOutput := ic.runModifyingInstructionsNew(codes, input)
	return lastOutput
}

// Modifies given slice
func (ic *IntCoder) runModifyingInstructionsNew(codes map[int]int, input int) int {
	defer func() {
		ic.idx = 0
		ic.relativeBase = 0
	}()
	outputs := make([]int, 0)
optLoop:
	for {
		execution := codes[ic.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[ic.getIdxFromMode(codes, execution, 3)] =
				codes[ic.getIdxFromMode(codes, execution, 2)] + codes[ic.getIdxFromMode(codes, execution, 1)]
			ic.idx += 4
		case 2:
			codes[ic.getIdxFromMode(codes, execution, 3)] = codes[ic.getIdxFromMode(codes, execution, 2)] * codes[ic.getIdxFromMode(codes, execution, 1)]
			ic.idx += 4
		case 3:
			codes[ic.getIdxFromMode(codes, execution, 1)] = input
			ic.idx += 2
		case 4:
			output := codes[ic.getIdxFromMode(codes, execution, 1)]
			outputs = append(outputs, output)
			ic.idx += 2
		case 5:
			if codes[ic.getIdxFromMode(codes, execution, 1)] != 0 {
				ic.idx = codes[ic.getIdxFromMode(codes, execution, 2)]
				break
			}
			ic.idx += 3
		case 6:
			if codes[ic.getIdxFromMode(codes, execution, 1)] == 0 {
				ic.idx = codes[ic.getIdxFromMode(codes, execution, 2)]
				break
			}
			ic.idx += 3
		case 7:
			var toAssign int
			if codes[ic.getIdxFromMode(codes, execution, 1)] < codes[ic.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[ic.getIdxFromMode(codes, execution, 3)] = toAssign
			ic.idx += 4
		case 8:
			var toAssign int
			if codes[ic.getIdxFromMode(codes, execution, 1)] == codes[ic.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[ic.getIdxFromMode(codes, execution, 3)] = toAssign
			ic.idx += 4
		case 9:
			ic.relativeBase += codes[ic.getIdxFromMode(codes, execution, 1)]
			ic.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}

	return outputs[len(outputs)-1]
}

func (ic *IntCoder) getIdxFromMode(codes map[int]int, execution int, parameterPosition int) int {
	mode := execution / int(math.Pow(10, 1+float64(parameterPosition)))
	mode %= 10
	switch mode {
	case 0:
		return codes[ic.idx+parameterPosition]
	case 1:
		return ic.idx + parameterPosition
	case 2:
		return ic.relativeBase + codes[ic.idx+parameterPosition]
	default:
		panic(fmt.Sprintf("Mode not known: %d", mode))
	}
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
