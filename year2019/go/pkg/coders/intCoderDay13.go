package coders

import (
	"fmt"
)

type ArcadeIntCoder struct {
	IntCoder
}

func (aic *ArcadeIntCoder) PlayArcade(codesInput []int, input int) map[Coordinate]int {
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	defer func() {
		aic.idx = 0
		aic.relativeBase = 0
	}()
	outputs := map[Coordinate]int{}
	outputCount := 0
	var x, y int
optLoop:
	for {
		execution := codes[aic.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[aic.getIdxFromMode(codes, execution, 3)] =
				codes[aic.getIdxFromMode(codes, execution, 2)] + codes[aic.getIdxFromMode(codes, execution, 1)]
			aic.idx += 4
		case 2:
			bar := codes[aic.getIdxFromMode(codes, execution, 2)] * codes[aic.getIdxFromMode(codes, execution, 1)]
			newIdx := aic.getIdxFromMode(codes, execution, 3)
			codes[newIdx] = bar

			aic.idx += 4
		case 3:
			codes[aic.getIdxFromMode(codes, execution, 1)] = input
			aic.idx += 2
		case 4:
			output := codes[aic.getIdxFromMode(codes, execution, 1)]
			switch outputInstruction := outputCount % 3; outputInstruction {
			case 0:
				x = output
			case 1:
				y = output
			case 2:
				outputs[Coordinate{x, y}] = output
			}
			outputCount++
			aic.idx += 2
		case 5:
			if codes[aic.getIdxFromMode(codes, execution, 1)] != 0 {
				aic.idx = codes[aic.getIdxFromMode(codes, execution, 2)]
				break
			}
			aic.idx += 3
		case 6:
			if codes[aic.getIdxFromMode(codes, execution, 1)] == 0 {
				aic.idx = codes[aic.getIdxFromMode(codes, execution, 2)]
				break
			}
			aic.idx += 3
		case 7:
			var toAssign int
			if codes[aic.getIdxFromMode(codes, execution, 1)] < codes[aic.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[aic.getIdxFromMode(codes, execution, 3)] = toAssign
			aic.idx += 4
		case 8:
			var toAssign int
			if codes[aic.getIdxFromMode(codes, execution, 1)] == codes[aic.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[aic.getIdxFromMode(codes, execution, 3)] = toAssign
			aic.idx += 4
		case 9:
			aic.relativeBase += codes[aic.getIdxFromMode(codes, execution, 1)]
			aic.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}
	return outputs
}
