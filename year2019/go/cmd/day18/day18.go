package main

import (
	"container/heap"
	"errors"
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	lines := createLines("day18")

	areaDefinition := createAreaDefinition(lines)
	keyPathFinder := pathFinder{}

	collector, err := keyPathFinder.findShortestPath(areaDefinition)

	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", collector.steps)
}

func createLines(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s_data.txt", fileName))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")
	return lines
}

type pathFinder struct{}

func (pf pathFinder) findShortestPath(areaDefinition areaDefinition) (*keyCollector, error) {
	keyCollector := createKeyCollector(areaDefinition.areaMap, areaDefinition.startingPoint)

	pq := make(PriorityQueue, 1)
	pq[0] = &Item{
		value:    keyCollector,
		priority: keyCollector.steps,
		index:    1,
	}

	heap.Init(&pq)

	for pq.Len() > 0 {
		item := heap.Pop(&pq).(*Item)
		collector := item.value

		current := collector.areaMap[collector.currentPosition]

		if unicode.IsUpper(current) && !collector.keysFound[current] {
			continue
		}

		if unicode.IsLower(current) {
			collector.keysFound[unicode.ToUpper(current)] = true
			collector.keysFoundCount++
			collector.visitedSinceLastKey = map[coord]bool{}
		}

		if collector.keysFoundCount == areaDefinition.keysInMap {
			return collector, nil
		}

		collector.areaMap[collector.currentPosition] = '.'

		neighbors := collector.getNeighbors()
		for _, neighbor := range neighbors {
			copied := collector.copy(neighbor)
			heap.Push(&pq, &Item{
				value:    copied,
				priority: copied.steps,
			})
		}
	}
	return nil, errors.New("not able to find path")
}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
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

type areaDefinition struct {
	areaMap       map[coord]rune
	startingPoint coord
	keysInMap     int
}

func createAreaDefinition(lines []string) areaDefinition {
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
