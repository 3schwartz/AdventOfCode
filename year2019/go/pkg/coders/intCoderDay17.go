package coders

import "fmt"

type ASCIIIntCoder struct {
	IntCoder
}

func (ASCIIIntCoder) FindScaffoldIntersectionsAboveThreshold(cameraMap map[Coordinate]int, threshold int) int {
	var sum int
	for coordinate, v := range cameraMap {
		if v != 35 {
			continue
		}
		isIntersect := true
		for _, movement := range GetMovements() {
			neighBoor, ok := cameraMap[Coordinate{coordinate.x + movement.Move.x, coordinate.y + movement.Move.y}]
			if !ok || neighBoor != 35 {
				isIntersect = false
				break
			}
		}
		if isIntersect {
			sum += coordinate.x * coordinate.y
		}
	}
	return sum
}

func (ascii *ASCIIIntCoder) createCodes(codesAsStrings []string) map[int]int {
	codesInput := ParseIntCodes(codesAsStrings)
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	return codes
}

func (ascii *ASCIIIntCoder) CreateCameraMap(codesAsStrings []string) map[Coordinate]int {
	codes := ascii.createCodes(codesAsStrings)
	currentPosition := Coordinate{}
	outputMap := map[Coordinate]int{}
optLoop:
	for {
		execution := codes[ascii.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[ascii.getIdxFromMode(codes, execution, 3)] =
				codes[ascii.getIdxFromMode(codes, execution, 2)] + codes[ascii.getIdxFromMode(codes, execution, 1)]
			ascii.idx += 4
		case 2:
			codes[ascii.getIdxFromMode(codes, execution, 3)] = codes[ascii.getIdxFromMode(codes, execution, 2)] * codes[ascii.getIdxFromMode(codes, execution, 1)]
			ascii.idx += 4
		case 3:
			panic("shouldn't be called")
		case 4:
			output := codes[ascii.getIdxFromMode(codes, execution, 1)]
			switch output {
			case 10:
				currentPosition = Coordinate{0, currentPosition.y + 1}
			default:
				existing, ok := outputMap[currentPosition]
				if ok && existing != output {
					panic(fmt.Sprintf("old was %d and new was %d as point x: %d, y: %d",
						existing, output, currentPosition.x, currentPosition.y))
				}
				outputMap[currentPosition] = output
				currentPosition = Coordinate{currentPosition.x + 1, currentPosition.y}
			}

			ascii.idx += 2
		case 5:
			if codes[ascii.getIdxFromMode(codes, execution, 1)] != 0 {
				ascii.idx = codes[ascii.getIdxFromMode(codes, execution, 2)]
				break
			}
			ascii.idx += 3
		case 6:
			if codes[ascii.getIdxFromMode(codes, execution, 1)] == 0 {
				ascii.idx = codes[ascii.getIdxFromMode(codes, execution, 2)]
				break
			}
			ascii.idx += 3
		case 7:
			var toAssign int
			if codes[ascii.getIdxFromMode(codes, execution, 1)] < codes[ascii.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[ascii.getIdxFromMode(codes, execution, 3)] = toAssign
			ascii.idx += 4
		case 8:
			var toAssign int
			if codes[ascii.getIdxFromMode(codes, execution, 1)] == codes[ascii.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[ascii.getIdxFromMode(codes, execution, 3)] = toAssign
			ascii.idx += 4
		case 9:
			ascii.relativeBase += codes[ascii.getIdxFromMode(codes, execution, 1)]
			ascii.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}

	return outputMap
}
