package main

import "unicode"

func main() {

}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

func (kc *keyCollector) getNeighbors() []coord {
	movements := [4]coord{
		{-1, 0}, {1, 0}, {0, 1}, {0, -1},
	}

	neighbors := make([]coord, 0)
	for _, m := range movements {
		neighbor := kc.currentPosition.add(m)
		visited := kc.visitedSinceLastKey[neighbor]
		if visited {
			continue
		}
		point, ok := kc.areaMap[neighbor]
		if ok && point != '#' {
			neighbors = append(neighbors, neighbor)
		}
	}

	return neighbors
}

func (kc *keyCollector) copy(newPosition coord) *keyCollector {
	areaMapCopy := map[coord]rune{}
	for key, value := range kc.areaMap {
		areaMapCopy[key] = value
	}
	visitedCopy := map[coord]bool{}
	for key, value := range kc.visitedSinceLastKey {
		visitedCopy[key] = value
	}
	keysFoundCopy := map[rune]bool{}
	for key, value := range kc.keysFound {
		keysFoundCopy[key] = value
	}

	visitedCopy[kc.currentPosition] = true

	return &keyCollector{
		areaMap:             areaMapCopy,
		currentPosition:     newPosition,
		steps:               kc.steps + 1,
		visitedSinceLastKey: visitedCopy,
		keysFoundCount:      kc.keysFoundCount,
		keysFound:           keysFoundCopy,
	}
}

type keyCollector struct {
	areaMap             map[coord]rune
	currentPosition     coord
	steps               int
	visitedSinceLastKey map[coord]bool
	keysFoundCount      int
	keysFound           map[rune]bool
}

func createKeyCollector(areaMap map[coord]rune, startingPosition coord) *keyCollector {
	return &keyCollector{
		areaMap:             areaMap,
		currentPosition:     startingPosition,
		visitedSinceLastKey: map[coord]bool{},
	}
}

type areaDefinition struct {
	areaMap       map[coord]rune
	startingPoint coord
	keysInMap     int
}

func createAreaMap(lines []string) areaDefinition {
	areaMap := map[coord]rune{}
	var keys int
	var startingPoint coord
	for n, line := range lines {
		for m, v := range line {
			if v == '#' {
				continue
			}
			if unicode.IsLower(v) {
				keys++
			}
			if v == '@' {
				startingPoint = coord{m, n}
			}
			areaMap[coord{m, n}] = v
		}
	}
	return areaDefinition{areaMap: areaMap, startingPoint: startingPoint, keysInMap: keys}
}
