package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	a := Coordinate{}
	b := Coordinate{}
	fmt.Println(a == b)
	// wireOne, wireTwo := readData()
	// fmt.Println(data)
}

type Move struct {
	Direction string
	Step      int
}

func NewMove(movement string) Move {
	// steps := make([]int, len(movement)-1)
	// for i, v := range movement[1:] {
	// 	step, err := strconv.Atoi(string(v))
	// 	if err != nil {
	// 		panic(err)
	// 	}
	// 	steps[i] = step
	// }
	step, err := strconv.Atoi(string(movement[1:]))
	if err != nil {
		panic(err)
	}
	return Move{
		Direction: movement[:1],
		Step:      step,
	}
}

type Wire struct {
	Moves []Move
}

func NewWire(moves []string) Wire {
	movements := make([]Move, 0, len(moves))
	for _, v := range moves {
		movements = append(movements, NewMove(v))
	}
	return Wire{
		Moves: movements,
	}

}

type Coordinate struct {
	x int
	y int
}

type GridVisits map[Coordinate]int

type Coordinates map[string]int

type ICalculator interface {
	CalculateCoordinates(wire Wire) GridVisits
	AddRange(coordinates Coordinates, coord string, value int, positive bool, places GridVisits)
}

type CoordinateCalculator struct {
}

func (cc CoordinateCalculator) CalculateCoordinates(wire Wire) GridVisits {
	coordinates := Coordinates{"x": 0, "y": 0, "s": 0}
	places := GridVisits{}

	for _, moves := range wire.Moves {
		switch direction := moves.Direction; direction {
		case "U":
			cc.AddRange(coordinates, "x", moves.Step, true, places)
		case "D":
			cc.AddRange(coordinates, "x", moves.Step, false, places)
		case "R":
			cc.AddRange(coordinates, "y", moves.Step, true, places)
		case "L":
			cc.AddRange(coordinates, "y", moves.Step, false, places)
		default:
			panic(fmt.Errorf("unknown direction %s", direction))
		}
	}
	return places
}

func (cc CoordinateCalculator) AddRange(coordinates Coordinates, coord string, value int, positive bool, places GridVisits) {
	multiplier := -1
	if positive {
		multiplier = 1
	}

	before := coordinates[coord]
	after := before + multiplier*value
	steps := coordinates["s"] + 1

	for i := before + multiplier; i < after+multiplier; i += multiplier {
		switch coord {
		case "x":
			places[Coordinate{i, coordinates["y"]}] = steps
		case "y":
			places[Coordinate{i, coordinates["y"]}] = steps
		default:
			panic(fmt.Sprintf("Unknown coordinate: %s", coord))
		}
		steps += 1
	}

	switch coord {
	case "x":
		coordinates["x"] += multiplier * value
	case "y":
		coordinates["x"] += multiplier * value
	default:
		panic(fmt.Sprintf("Unknown coordinate: %s", coord))
	}
	coordinates["s"] += value
}

type MinimumCalculator struct {
	Calculator ICalculator
	WireOne    Wire
	WireTwo    Wire
	VisitsOne  GridVisits
	VisitsTwo  GridVisits
}

func NewMinimumCalculator(wireOneInput []string, wireSecondInput []string) MinimumCalculator {
	calculator := CoordinateCalculator{}
	wireOne := NewWire(wireOneInput)
	wireTwo := NewWire(wireSecondInput)
	wireOneVisited := calculator.CalculateCoordinates(wireOne)
	wireTwoVisited := calculator.CalculateCoordinates(wireTwo)
	return MinimumCalculator{
		calculator,
		wireOne, wireTwo, wireOneVisited, wireTwoVisited,
	}
}

func (mc *MinimumCalculator) GetIntersection() []Coordinate {
	bucket := map[Coordinate]bool{}
	toDelete := []Coordinate{}
	for key := range mc.VisitsOne {
		bucket[key] = true
	}
	for key := range mc.VisitsTwo {
		if bucket[key] {
			continue
		}
		toDelete = append(toDelete, key)
	}
	for _, key := range toDelete {
		delete(bucket, key)
	}
	intersection := make([]Coordinate, 0, len(bucket))
	for key := range bucket {
		intersection = append(intersection, key)
	}
	return intersection
}

func readData() ([]string, []string) {
	f, err := os.ReadFile("../../../data/day3_data.txt")
	if err != nil {
		panic(err)
	}
	trimmed := strings.Trim(string(f), "\n")
	data := strings.Split(trimmed, "\n")

	if length := len(data); length != 2 {
		panic(fmt.Sprintf("Length should be 2 was %d", length))
	}

	wireOne := strings.Split(data[0], ",")
	wireTwo := strings.Split(data[1], ",")
	return wireOne, wireTwo
}
