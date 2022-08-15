package coders

import (
	"fmt"
)

type gameState struct {
	TotalScore int
	BlockCount int
	paddleX    int
	ballX      int
	x, y       int
}

type ArcadeIntCoder struct {
	IntCoder
}

func (aic *ArcadeIntCoder) PlayArcade(codesInput []int) gameState {
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	defer func() {
		aic.idx = 0
		aic.relativeBase = 0
	}()
	state := gameState{}
	outputCount := 0
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
			var input int
			if state.ballX < state.paddleX {
				input = -1
			}
			if state.ballX > state.paddleX {
				input = 1
			}
			codes[aic.getIdxFromMode(codes, execution, 1)] = input
			aic.idx += 2
		case 4:
			output := codes[aic.getIdxFromMode(codes, execution, 1)]
			state = aic.evaluateOutput(state, outputCount, output)
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
	return state
}

func (aic *ArcadeIntCoder) evaluateOutput(state gameState, outputCount int, output int) gameState {
	switch outputInstruction := outputCount % 3; outputInstruction {
	case 0:
		state.x = output
	case 1:
		state.y = output
	case 2:
		if state.x == -1 && state.y == 0 {
			state.TotalScore = output
			fmt.Printf("Block count: %d, with total score: %d\n", state.BlockCount, state.TotalScore)
			break
		}
		switch output {
		case 2:
			state.BlockCount++
		case 3:
			state.paddleX = state.x
		case 4:
			state.ballX = state.x
		}
	}
	return state
}
