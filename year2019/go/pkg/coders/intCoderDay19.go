package coders

import (
	"fmt"
)

type Coord struct {
	X int
	Y int
}

func (c Coord) FindNeighbors() map[Coord]struct{} {
	neighbors := map[Coord]struct{}{}
	neighbors[Coord{c.X + 1, c.Y}] = struct{}{}
	neighbors[Coord{c.X, c.Y + 1}] = struct{}{}
	neighbors[Coord{c.X + 1, c.Y + 1}] = struct{}{}
	return neighbors
}

type BeamPoint struct {
	Coord
	Pulled int
}

type TractorBeamIntCoder struct {
	IntCoder
}

func (tb *TractorBeamIntCoder) FindPointsAffected(codesInput []int, out chan<- BeamPoint, size int) {
	defer func() {
		close(out)
	}()
	for x := 0; x < size; x++ {
		for y := 0; y < size; y++ {
			codes := tb.GenerateCodes(codesInput)
			tb.pointAffected(codes, x, y, out)
		}
	}
}

func (tb *TractorBeamIntCoder) FindNeighborsAffected(
	codesInput []int,
	neighbors map[Coord]struct{},
	out chan<- BeamPoint) {
	defer func() {
		close(out)
	}()
	for neighbor := range neighbors {
		codes := tb.GenerateCodes(codesInput)
		tb.pointAffected(codes, neighbor.X, neighbor.Y, out)
	}
}

func (tb *TractorBeamIntCoder) pointAffected(codes map[int]int, x int, y int, out chan<- BeamPoint) {
	defer func() {
		tb.idx = 0
		tb.relativeBase = 0
	}()
	var xOrY int
optLoop:
	for {
		execution := codes[tb.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			codes[tb.getIdxFromMode(codes, execution, 3)] =
				codes[tb.getIdxFromMode(codes, execution, 2)] + codes[tb.getIdxFromMode(codes, execution, 1)]
			tb.idx += 4
		case 2:
			codes[tb.getIdxFromMode(codes, execution, 3)] = codes[tb.getIdxFromMode(codes, execution, 2)] * codes[tb.getIdxFromMode(codes, execution, 1)]
			tb.idx += 4
		case 3:
			switch xOrY {
			case 0:
				codes[tb.getIdxFromMode(codes, execution, 1)] = x
				xOrY = 1
			case 1:
				codes[tb.getIdxFromMode(codes, execution, 1)] = y
				xOrY = 0
			}
			tb.idx += 2
		case 4:
			output := codes[tb.getIdxFromMode(codes, execution, 1)]
			tb.idx += 2
			out <- BeamPoint{
				Coord:  Coord{x, y},
				Pulled: output,
			}
			return
		case 5:
			if codes[tb.getIdxFromMode(codes, execution, 1)] != 0 {
				tb.idx = codes[tb.getIdxFromMode(codes, execution, 2)]
				break
			}
			tb.idx += 3
		case 6:
			if codes[tb.getIdxFromMode(codes, execution, 1)] == 0 {
				tb.idx = codes[tb.getIdxFromMode(codes, execution, 2)]
				break
			}
			tb.idx += 3
		case 7:
			var toAssign int
			if codes[tb.getIdxFromMode(codes, execution, 1)] < codes[tb.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[tb.getIdxFromMode(codes, execution, 3)] = toAssign
			tb.idx += 4
		case 8:
			var toAssign int
			if codes[tb.getIdxFromMode(codes, execution, 1)] == codes[tb.getIdxFromMode(codes, execution, 2)] {
				toAssign = 1
			}
			codes[tb.getIdxFromMode(codes, execution, 3)] = toAssign
			tb.idx += 4
		case 9:
			tb.relativeBase += codes[tb.getIdxFromMode(codes, execution, 1)]
			tb.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}
}
