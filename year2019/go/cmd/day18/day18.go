package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	lines := createLines("day18")

	areaDefinition := createAreaDefinition(lines)
	keyPathFinder := pathGraphFinder{}

	steps, err := keyPathFinder.findShortestPath(areaDefinition)

	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", steps)
}

type pathFinder interface {
	findShortestPath(definition areaDefinition) (int, error)
}

func createLines(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s_data.txt", fileName))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")
	return lines
}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

func (c coord) getNeighbors() [4]coord {
	movements := [4]coord{
		{-1, 0}, {1, 0}, {0, 1}, {0, -1},
	}

	neighbors := [4]coord{}
	for i, m := range movements {
		neighbors[i] = c.add(m)
	}
	return neighbors
}

type areaDefinition struct {
	areaMap                 map[coord]rune
	startingPoint           coord
	keysInMap               int
	keysAsBitRepresentation int
}

func createAreaDefinition(lines []string) areaDefinition {
	areaMap := map[coord]rune{}
	var keys int
	var startingPoint coord
	var keysAsBitRepresentation int
	for n, line := range lines {
		for m, v := range line {
			if v == '#' {
				continue
			}
			if unicode.IsLower(v) {
				keys++
				shift := v - 'a'
				keysAsBitRepresentation |= 1 << shift
			}
			if v == '@' {
				startingPoint = coord{m, n}
			}
			areaMap[coord{m, n}] = v
		}
	}
	return areaDefinition{
		areaMap:                 areaMap,
		startingPoint:           startingPoint,
		keysInMap:               keys,
		keysAsBitRepresentation: keysAsBitRepresentation,
	}
}
