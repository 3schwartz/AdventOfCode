package coders

import (
	"fmt"
	"math"
)

type Coordinate struct {
	x int
	y int
}

func NewCoordinate(x int, y int) Coordinate {
	return Coordinate{x, y}
}

type PaintHullIntCoder struct {
	IntCoder
}

func (ph *PaintHullIntCoder) OutputHullPaint(visited map[Coordinate]int, writer func(output []string)) {
	xMin, yMin := math.MaxInt, math.MaxInt
	xMax, yMax := math.MinInt, math.MinInt
	for coord := range visited {
		if coord.x > xMax {
			xMax = coord.x
		}
		if coord.y > yMax {
			yMax = coord.y
		}
		if coord.x < xMin {
			xMin = coord.x
		}
		if coord.y < yMin {
			yMin = coord.y
		}
	}
	var xLength = math.Abs(float64(xMin)) + math.Abs(float64(xMax)) + 1
	var yLength = math.Abs(float64(yMin)) + math.Abs(float64(yMax)) + 1
	rows := make([]string, int(yLength))
	var output string
	for i := 0; i < int(xLength); i++ {
		for j := 0; j < int(yLength); j++ {
			output = "."
			if visited[Coordinate{i - int(math.Abs(math.Abs(float64(xMin)))), j - int(math.Abs(math.Abs(float64(yMin))))}] == 1 {
				output = "#"
			}
			rows[j] = output
		}
		writer(rows)
		// fmt.Println(rows)
	}
}

func (ph *PaintHullIntCoder) PaintHull(codes []int) map[Coordinate]int {
	visited := map[Coordinate]int{}
	ph.PaintHullWithInput(codes, visited)
	return visited
}

func (ph *PaintHullIntCoder) findActionFromOutput(output int) int {
	switch output {
	case 0:
		return 0
	case 1:
		return 1
	}
	panic(fmt.Sprintf("Output not recognized: %d", output))
}

func (ph *PaintHullIntCoder) PaintHullWithInput(codesInput []int, visited map[Coordinate]int) {
	defer func() {
		ph.idx = 0
		ph.relativeBase = 0
	}()
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	currentPosition := Coordinate{0, 0}
	direction := Coordinate{0, 1}
	outputCallCount := 0
optLoop:
	for {
		execution := codes[ph.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[ph.getIdxFromMode(codes, execution, 3)] =
				codes[ph.getIdxFromMode(codes, execution, 2)] + codes[ph.getIdxFromMode(codes, execution, 1)]
			ph.idx += 4
		case 2:
			bar := codes[ph.getIdxFromMode(codes, execution, 2)] * codes[ph.getIdxFromMode(codes, execution, 1)]
			newIdx := ph.getIdxFromMode(codes, execution, 3)
			codes[newIdx] = bar

			ph.idx += 4
		case 3:
			codes[ph.getIdxFromMode(codes, execution, 1)] = visited[currentPosition]
			ph.idx += 2
		case 4:
			outputCallCount++
			output := codes[ph.getIdxFromMode(codes, execution, 1)]
			ph.idx += 2
			action := ph.findActionFromOutput(output)
			if outputCallCount%2 == 1 {
				visited[currentPosition] = action
				continue
			}
			switch action {
			case 0:
				direction = Coordinate{-direction.y, direction.x}
			case 1:
				direction = Coordinate{direction.y, -direction.x}
			}
			currentPosition = Coordinate{currentPosition.x + direction.x, currentPosition.y + direction.y}
		case 5:
			if codes[ph.getIdxFromMode(codes, execution, 1)] != 0 {
				ph.idx = codes[ph.getIdxFromMode(codes, execution, 2)]
				break
			}
			ph.idx += 3
		case 6:
			if codes[ph.getIdxFromMode(codes, execution, 1)] == 0 {
				ph.idx = codes[ph.getIdxFromMode(codes, execution, 2)]
				break
			}
			ph.idx += 3
		case 7:
			var toAssign int
			if codes[ph.getIdxFromMode(codes, execution, 1)] < codes[ph.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[ph.getIdxFromMode(codes, execution, 3)] = toAssign
			ph.idx += 4
		case 8:
			var toAssign int
			if codes[ph.getIdxFromMode(codes, execution, 1)] == codes[ph.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[ph.getIdxFromMode(codes, execution, 3)] = toAssign
			ph.idx += 4
		case 9:
			ph.relativeBase += codes[ph.getIdxFromMode(codes, execution, 1)]
			ph.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}
}
