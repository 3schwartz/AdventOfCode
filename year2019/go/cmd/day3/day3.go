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

type gridVisits map[coordinate]int

type coordinates map[string]int

type iCalculator interface {
	calculateCoordinates(wire wire) gridVisits
	addRange(coordinates coordinates, coord string, value int, positive bool, places gridVisits)
}

type coordinateCalculator struct {
}

func (cc coordinateCalculator) calculateCoordinates(wire wire) gridVisits {
	coordinates := coordinates{"x": 0, "y": 0, "s": 0}
	places := gridVisits{}

	for _, moves := range wire.Moves {
		switch direction := moves.Direction; direction {
		case "U":
			cc.addRange(coordinates, "x", moves.Step, true, places)
		case "D":
			cc.addRange(coordinates, "x", moves.Step, false, places)
		case "R":
			cc.addRange(coordinates, "y", moves.Step, true, places)
		case "L":
			cc.addRange(coordinates, "y", moves.Step, false, places)
		default:
			panic(fmt.Errorf("unknown direction %s", direction))
		}
	}
	return places
}

func (cc coordinateCalculator) directionConditionComparer(pos int, after int, multiplier int) bool {
	if multiplier > 0 {
		return pos < after+multiplier
	}
	return pos > after+multiplier
}

func (cc coordinateCalculator) addRange(coordinates coordinates, coord string, value int, positive bool, places gridVisits) {
	multiplier := -1
	if positive {
		multiplier = 1
	}

	before := coordinates[coord]
	after := before + multiplier*value
	steps := coordinates["s"] + 1

	for i := before + multiplier; cc.directionConditionComparer(i, after, multiplier); i += multiplier {
		switch coord {
		case "x":
			places[coordinate{i, coordinates["y"]}] = steps
		case "y":
			places[coordinate{coordinates["x"], i}] = steps
		default:
			panic(fmt.Sprintf("Unknown coordinate: %s", coord))
		}
		steps += 1
	}

	switch coord {
	case "x":
		coordinates["x"] += multiplier * value
	case "y":
		coordinates["y"] += multiplier * value
	default:
		panic(fmt.Sprintf("Unknown coordinate: %s", coord))
	}
	coordinates["s"] += value
}

type minimumCalculator struct {
	Calculator iCalculator
	WireOne    wire
	WireTwo    wire
	VisitsOne  gridVisits
	VisitsTwo  gridVisits
}

func newMinimumCalculator(wireOneInput []string, wireSecondInput []string) minimumCalculator {
	calculator := coordinateCalculator{}
	wireOne := newWire(wireOneInput)
	wireTwo := newWire(wireSecondInput)
	wireOneVisited := calculator.calculateCoordinates(wireOne)
	wireTwoVisited := calculator.calculateCoordinates(wireTwo)
	return minimumCalculator{
		calculator,
		wireOne, wireTwo, wireOneVisited, wireTwoVisited,
	}
}

func (mc *minimumCalculator) getMinimumDistance() int {
	intersections := mc.getIntersection()
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
	intersections := mc.getIntersection()
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

func (mc *minimumCalculator) getIntersection() []coordinate {
	bucket := map[coordinate]bool{}
	intersect := []coordinate{}
	for key := range mc.VisitsOne {
		bucket[key] = true
	}
	for key := range mc.VisitsTwo {
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
