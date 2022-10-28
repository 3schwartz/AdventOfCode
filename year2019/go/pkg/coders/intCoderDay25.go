package coders

import (
	"context"
	"fmt"
)

type DroidCoder struct {
	IntCoder
	inputChan  <-chan int
	outputChan chan<- int
}

func CreateDroidCoder() (*DroidCoder, chan<- int, <-chan int) {
	inputChannel := make(chan int, 0)
	outputChannel := make(chan int, 0)
	return &DroidCoder{
		inputChan:  inputChannel,
		outputChan: outputChannel,
	}, inputChannel, outputChannel
}

func (dc *DroidCoder) FindSanta(codes map[int]int, context context.Context) {

optLoop:
	for {
		if context.Err() != nil {
			break
		}
		execution := codes[dc.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[dc.getIdxFromMode(codes, execution, 3)] =
				codes[dc.getIdxFromMode(codes, execution, 2)] + codes[dc.getIdxFromMode(codes, execution, 1)]
			dc.idx += 4
		case 2:
			codes[dc.getIdxFromMode(codes, execution, 3)] = codes[dc.getIdxFromMode(codes, execution, 2)] * codes[dc.getIdxFromMode(codes, execution, 1)]
			dc.idx += 4
		case 3:
			input := <-dc.inputChan
			codes[dc.getIdxFromMode(codes, execution, 1)] = input
			dc.idx += 2
		case 4:
			output := codes[dc.getIdxFromMode(codes, execution, 1)]
			dc.idx += 2
			dc.outputChan <- output
		case 5:
			if codes[dc.getIdxFromMode(codes, execution, 1)] != 0 {
				dc.idx = codes[dc.getIdxFromMode(codes, execution, 2)]
				break
			}
			dc.idx += 3
		case 6:
			if codes[dc.getIdxFromMode(codes, execution, 1)] == 0 {
				dc.idx = codes[dc.getIdxFromMode(codes, execution, 2)]
				break
			}
			dc.idx += 3
		case 7:
			var toAssign int
			if codes[dc.getIdxFromMode(codes, execution, 1)] < codes[dc.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[dc.getIdxFromMode(codes, execution, 3)] = toAssign
			dc.idx += 4
		case 8:
			var toAssign int
			if codes[dc.getIdxFromMode(codes, execution, 1)] == codes[dc.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[dc.getIdxFromMode(codes, execution, 3)] = toAssign
			dc.idx += 4
		case 9:
			dc.relativeBase += codes[dc.getIdxFromMode(codes, execution, 1)]
			dc.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}
}
