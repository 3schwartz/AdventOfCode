package coders

import (
	"context"
	"fmt"
)

type sendAddressState int

const (
	Address sendAddressState = iota
	X
	Y
)

type outputCoordinate struct {
	x       int
	y       int
	address int
}

type NetworkCoder struct {
	IntCoder
	initialInput        int
	inputIsUsed         bool
	getCoordinate       func(int) (bool, Coordinate)
	sendCoordinate      func(int, Coordinate)
	useCachedCoordinate bool
	cachedCoordinate    Coordinate
	sendState           sendAddressState
	coordinateToSend    outputCoordinate
}

func (nc *NetworkCoder) sendOutput(output int) {
	switch nc.sendState {
	case Address:
		nc.coordinateToSend.address = output
		nc.sendState = X
	case X:
		nc.coordinateToSend.x = output
		nc.sendState = Y
	case Y:
		nc.coordinateToSend.y = output
		nc.sendCoordinate(nc.coordinateToSend.address, Coordinate{
			x: nc.coordinateToSend.x,
			y: nc.coordinateToSend.y,
		})
		nc.coordinateToSend = outputCoordinate{}
		nc.sendState = Address
	}
}

func CreateNetworkCoder(initialInput int, getCoordinate func(int) (bool, Coordinate), sendCoordinate func(int, Coordinate)) *NetworkCoder {
	return &NetworkCoder{
		initialInput:   initialInput,
		getCoordinate:  getCoordinate,
		sendCoordinate: sendCoordinate,
	}
}

func (nc *NetworkCoder) createMap(codesInput []int) map[int]int {
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	return codes
}

func (nc *NetworkCoder) getInput() int {
	if !nc.inputIsUsed {
		nc.inputIsUsed = true
		return nc.initialInput
	}
	if nc.useCachedCoordinate {
		input := nc.cachedCoordinate.y
		nc.useCachedCoordinate = false
		return input
	}
	ok, coord := nc.getCoordinate(nc.initialInput)
	if !ok {
		return -1
	}
	nc.cachedCoordinate = coord
	nc.useCachedCoordinate = true
	return coord.x
}

func (nc *NetworkCoder) RunNetwork(codesInput []int, context context.Context) {
	codes := nc.createMap(codesInput)
optLoop:
	for {
		if context.Err() != nil {
			return
		}
		execution := codes[nc.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[nc.getIdxFromMode(codes, execution, 3)] =
				codes[nc.getIdxFromMode(codes, execution, 2)] + codes[nc.getIdxFromMode(codes, execution, 1)]
			nc.idx += 4
		case 2:
			codes[nc.getIdxFromMode(codes, execution, 3)] = codes[nc.getIdxFromMode(codes, execution, 2)] * codes[nc.getIdxFromMode(codes, execution, 1)]
			nc.idx += 4
		case 3:
			input := nc.getInput()
			codes[nc.getIdxFromMode(codes, execution, 1)] = input
			nc.idx += 2
		case 4:
			output := codes[nc.getIdxFromMode(codes, execution, 1)]
			nc.sendOutput(output)
			nc.idx += 2
		case 5:
			if codes[nc.getIdxFromMode(codes, execution, 1)] != 0 {
				nc.idx = codes[nc.getIdxFromMode(codes, execution, 2)]
				break
			}
			nc.idx += 3
		case 6:
			if codes[nc.getIdxFromMode(codes, execution, 1)] == 0 {
				nc.idx = codes[nc.getIdxFromMode(codes, execution, 2)]
				break
			}
			nc.idx += 3
		case 7:
			var toAssign int
			if codes[nc.getIdxFromMode(codes, execution, 1)] < codes[nc.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[nc.getIdxFromMode(codes, execution, 3)] = toAssign
			nc.idx += 4
		case 8:
			var toAssign int
			if codes[nc.getIdxFromMode(codes, execution, 1)] == codes[nc.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[nc.getIdxFromMode(codes, execution, 3)] = toAssign
			nc.idx += 4
		case 9:
			nc.relativeBase += codes[nc.getIdxFromMode(codes, execution, 1)]
			nc.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}

	return
}
