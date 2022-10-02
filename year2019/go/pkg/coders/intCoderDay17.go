package coders

import (
	"errors"
	"fmt"
	"strconv"
)

type ASCIIIntCoder struct {
	IntCoder
}

func (ASCIIIntCoder) findDirection(robot int) Coordinate {
	switch robot {
	case '^':
		return Coordinate{0, -1}
	case 'v':
		return Coordinate{0, 1}
	case '<':
		return Coordinate{-1, 0}
	case '>':
		return Coordinate{1, 0}
	default:
		panic(fmt.Sprintf("doesn't not robot rune: %d", robot))
	}
}

func (ascii ASCIIIntCoder) GetMovements(cameraMap map[Coordinate]int, robot int, robotPosition Coordinate) []string {
	direction := ascii.findDirection(robot)
	position := robotPosition
	var straightCount int
	movements := make([]string, 0)
	for {
		straight := position.Add(direction)
		if cameraMap[straight] == '#' {
			straightCount++
			position = straight
			continue
		}
		if straightCount != 0 {
			movements = append(movements, strconv.Itoa(straightCount))
			straightCount = 0
		}

		// Coordinate system in opposite direction
		leftRotate := Coordinate{direction.y, -1 * direction.x}
		if cameraMap[position.Add(leftRotate)] == '#' {
			movements = append(movements, "L")
			direction = leftRotate
			continue
		}
		rightRotate := Coordinate{-1 * direction.y, direction.x}
		if cameraMap[position.Add(rightRotate)] == '#' {
			movements = append(movements, "R")
			direction = rightRotate
			continue
		}
		break
	}
	return movements
}

func (ASCIIIntCoder) GetRobotPosition(cameraMap map[Coordinate]int) (int, Coordinate, error) {
	for key, value := range cameraMap {
		if value == '^' || value == '<' || value == '>' || value == 'v' {
			return value, key, nil
		}
	}
	return 0, Coordinate{}, errors.New("couldn't find robot")
}

func (ascii ASCIIIntCoder) Print(cameraMap map[Coordinate]int) {
	fmt.Print("\n")
	coord := Coordinate{}
	for {
		for {
			output, ok := cameraMap[coord]
			if !ok {
				break
			}
			switch output {
			case 35:
				fmt.Print("#")
			case 94:
				fmt.Print("^")
			case 46:
				fmt.Print(".")
			default:
				panic(fmt.Sprintf("output not known: %d", output))
			}

			coord = Coordinate{coord.x + 1, coord.y}
		}
		fmt.Print("\n")
		coord = Coordinate{0, coord.y + 1}
		_, hasNextLine := cameraMap[coord]
		if !hasNextLine {
			break
		}
	}
	fmt.Print("\n")
}

func (ascii ASCIIIntCoder) FindScaffoldIntersections(cameraMap map[Coordinate]int) int {
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
	defer func() {
		ascii.idx = 0
		ascii.relativeBase = 0
	}()
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

func (ascii *ASCIIIntCoder) ReportDust(codesAsStrings []string, input []int) (int, error) {
	defer func() {
		ascii.idx = 0
		ascii.relativeBase = 0
	}()
	var inputIdx int
	codes := ascii.createCodes(codesAsStrings)
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
			codes[ascii.getIdxFromMode(codes, execution, 1)] = input[inputIdx]
			ascii.idx += 2
			inputIdx++
		case 4:
			output := codes[ascii.getIdxFromMode(codes, execution, 1)]
			return output, nil
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

	return 0, errors.New("couldn't find output dust")
}
