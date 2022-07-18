package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	wireOne, wireTwo := readData()
	minimumCalculator := newMinimumCalculator(wireOne, wireTwo)
	minimumDistance := minimumCalculator.getMinimumDistance()

	fmt.Printf("Part 1: %d\n", minimumDistance)

	minimumSteps := minimumCalculator.getMinimumSteps()

	fmt.Printf("Part 2: %d\n", minimumSteps)
}

type move struct {
	Direction string
	Step      int
}

func newMove(movement string) move {
	step, err := strconv.Atoi(string(movement[1:]))
	if err != nil {
		panic(err)
	}
	return move{
		Direction: movement[:1],
		Step:      step,
	}
}

type wire struct {
	Moves []move
}

func newWire(moves []string) wire {
	movements := make([]move, 0, len(moves))
	for _, v := range moves {
		movements = append(movements, newMove(v))
	}
	return wire{
		Moves: movements,
	}

}

type coordinate struct {
	x int
	y int
}

type coordinateVisits map[coordinate]int

type position map[string]int

type coordinateMapper interface {
	mapCoordinates(wire wire) coordinateVisits
	addCoordinates(coordinates position, coord string, value int, positive bool, places coordinateVisits)
}

type wireCoordinateMapper struct {
}

func (cc wireCoordinateMapper) mapCoordinates(wire wire) coordinateVisits {
	currentPosition := position{"x": 0, "y": 0, "s": 0}
	places := coordinateVisits{}

	for _, moves := range wire.Moves {
		switch direction := moves.Direction; direction {
		case "U":
			cc.addCoordinates(currentPosition, "x", moves.Step, true, places)
		case "D":
			cc.addCoordinates(currentPosition, "x", moves.Step, false, places)
		case "R":
			cc.addCoordinates(currentPosition, "y", moves.Step, true, places)
		case "L":
			cc.addCoordinates(currentPosition, "y", moves.Step, false, places)
		default:
			panic(fmt.Errorf("unknown direction %s", direction))
		}
	}
	return places
}

func (cc wireCoordinateMapper) directionConditionComparer(pos int, after int, multiplier int) bool {
	if multiplier > 0 {
		return pos < after+multiplier
	}
	return pos > after+multiplier
}

// Change current position and visited input parameters
func (cc wireCoordinateMapper) addCoordinates(currentPosition position, coord string, value int, positive bool, visited coordinateVisits) {
	multiplier := -1
	if positive {
		multiplier = 1
	}

	before := currentPosition[coord]
	after := before + multiplier*value
	steps := currentPosition["s"] + 1

	for i := before + multiplier; cc.directionConditionComparer(i, after, multiplier); i += multiplier {
		switch coord {
		case "x":
			visited[coordinate{i, currentPosition["y"]}] = steps
		case "y":
			visited[coordinate{currentPosition["x"], i}] = steps
		default:
			panic(fmt.Sprintf("Unknown coordinate: %s", coord))
		}
		steps += 1
	}

	switch coord {
	case "x":
		currentPosition["x"] += multiplier * value
	case "y":
		currentPosition["y"] += multiplier * value
	default:
		panic(fmt.Sprintf("Unknown coordinate: %s", coord))
	}
	currentPosition["s"] += value
}

type minimumCalculator struct {
	Calculator coordinateMapper
	WireOne    wire
	WireTwo    wire
	VisitsOne  coordinateVisits
	VisitsTwo  coordinateVisits
}

func newMinimumCalculator(wireOneInput []string, wireSecondInput []string) minimumCalculator {
	calculator := wireCoordinateMapper{}
	wireOne := newWire(wireOneInput)
	wireTwo := newWire(wireSecondInput)
	wireOneVisited := calculator.mapCoordinates(wireOne)
	wireTwoVisited := calculator.mapCoordinates(wireTwo)
	return minimumCalculator{
		calculator,
		wireOne, wireTwo, wireOneVisited, wireTwoVisited,
	}
}

func (mc *minimumCalculator) getMinimumDistance() int {
	intersections := getIntersection(mc.VisitsOne, mc.VisitsTwo)
	if len(intersections) == 0 {
		panic("No valid intersections")
	}
	minValue := math.MaxInt32
	for _, intersect := range intersections {
		if intersect.x == 0 && intersect.y == 0 {
			continue
		}
		manhattanDistance := int(math.Abs(float64(intersect.x))) + int(math.Abs(float64(intersect.y)))
		if manhattanDistance < minValue {
			minValue = manhattanDistance
		}
	}
	return minValue
}

func (mc *minimumCalculator) getMinimumSteps() int {
	intersections := getIntersection(mc.VisitsOne, mc.VisitsTwo)
	if len(intersections) == 0 {
		panic("No valid intersections")
	}
	minValue := math.MaxInt32
	for _, intersect := range intersections {
		if intersect.x == 0 && intersect.y == 0 {
			continue
		}
		stepsOne, ok := mc.VisitsOne[intersect]
		if !ok {
			panic("Intersect not present in visited first")
		}
		stepsTwo, ok := mc.VisitsTwo[intersect]
		if !ok {
			panic("Intersect not present in visited second")
		}

		steps := stepsOne + stepsTwo
		if steps < minValue {
			minValue = steps
		}
	}
	return minValue
}

func getIntersection(visitOne coordinateVisits, visitSecond coordinateVisits) []coordinate {
	bucket := map[coordinate]bool{}
	intersect := []coordinate{}
	for key := range visitOne {
		bucket[key] = true
	}
	for key := range visitSecond {
		if bucket[key] {
			intersect = append(intersect, key)
		}
	}
	return intersect
}

func getIntersectionGeneric[T comparable, K1 any, K2 any](m1 map[T]K1, m2 map[T]K2) []T {
	bucket := map[T]bool{}
	intersect := []T{}
	for key := range m1 {
		bucket[key] = true
	}
	for key := range m2 {
		if bucket[key] {
			intersect = append(intersect, key)
		}
	}
	return intersect
}

func readData() ([]string, []string) {
	f, err := os.ReadFile("../../../data/day3_data.txt")
	if err != nil {
		panic(err)
	}
	trimmed := strings.Trim(string(f), "\r\n")
	data := strings.Split(trimmed, "\r\n")

	if length := len(data); length != 2 {
		panic(fmt.Sprintf("Length should be 2 was %d", length))
	}

	wireOne := strings.Split(data[0], ",")
	wireTwo := strings.Split(data[1], ",")
	return wireOne, wireTwo
}
