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

type graph struct {
	graph map[coord]map[rune]int
}

func createGraph(areaMap map[coord]rune) *graph {
	graph := graph{}
	for coord, symbol := range areaMap {
		if !unicode.IsLetter(symbol) {
			continue
		}
		coordNodes := graph.findCoordNodesInGraph(areaMap, coord)
		graph.graph[coord] = coordNodes
	}
	return &graph
}

type nodeQueueElements struct {
	coord coord
	steps int
}

func (g graph) findCoordNodesInGraph(areaMap map[coord]rune, currentCoord coord) map[rune]int {
	visited := map[coord]struct{}{}
	nodes := map[rune]int{}
	queue := createQueue[nodeQueueElements]()

	queue.append(nodeQueueElements{
		coord: currentCoord,
		steps: 0})
	visited[currentCoord] = struct{}{}
	for {
		current, moreElements := queue.tryDequeue()
		if !moreElements {
			break
		}
		for _, neighbor := range current.coord.getNeighbors() {

		}

	}

	return nodes

}

type coordVisited struct {
	coord     coord
	keysAsBit int
}

type pathFinder struct{}

func (pf pathFinder) findBitwiseKeyShift(key rune, currentKeys int) int {
	amountToShift := key - 'a'
	keyBit := 1 << amountToShift
	currentKeys |= keyBit
	return currentKeys
}

func (pf pathFinder) findShortestPath(areaDefinition areaDefinition) (*keyCollector, error) {
	keyCollector := createKeyCollector(areaDefinition.areaMap, areaDefinition.startingPoint)

	pq := make(PriorityQueue, 1)
	pq[0] = &Item{
		value:    keyCollector,
		priority: keyCollector.steps,
		index:    1,
	}

	visited := map[coordVisited]int{}

	heap.Init(&pq)

	for pq.Len() > 0 {
		item := heap.Pop(&pq).(*Item)
		collector := item.value

		if previous, ok := visited[coordVisited{collector.currentPosition, collector.keysFoundBitRepresentation}]; ok && previous < collector.steps {
			continue
		}
		visited[coordVisited{collector.currentPosition, collector.keysFoundBitRepresentation}] = collector.steps

		current := collector.areaMap[collector.currentPosition]

		if unicode.IsUpper(current) && !collector.keysFound[current] {
			continue
		}

		if unicode.IsLower(current) {
			collector.keysFound[unicode.ToUpper(current)] = true
			collector.keysFoundCount++
			collector.visitedSinceLastKey = map[coord]bool{}
			collector.keysFoundBitRepresentation = pf.findBitwiseKeyShift(current, collector.keysFoundBitRepresentation)
		}

		if collector.keysFoundCount == areaDefinition.keysInMap {
			return collector, nil
		}

		collector.areaMap[collector.currentPosition] = '.'

		neighbors := collector.getNeighbors(visited)
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

type keyCollector struct {
	areaMap                    map[coord]rune
	currentPosition            coord
	steps                      int
	visitedSinceLastKey        map[coord]bool
	keysFoundCount             int
	keysFound                  map[rune]bool
	keysFoundBitRepresentation int
}

func createKeyCollector(areaMap map[coord]rune, startingPosition coord) *keyCollector {
	return &keyCollector{
		areaMap:             areaMap,
		currentPosition:     startingPosition,
		visitedSinceLastKey: map[coord]bool{},
	}
}

func (kc *keyCollector) getNeighbors(visited map[coordVisited]int) []coord {
	currentNeighbors := kc.currentPosition.getNeighbors()

	neighbors := make([]coord, 0)
	for _, neighbor := range currentNeighbors {
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
		areaMap:                    areaMapCopy,
		currentPosition:            newPosition,
		steps:                      kc.steps + 1,
		visitedSinceLastKey:        visitedCopy,
		keysFoundCount:             kc.keysFoundCount,
		keysFound:                  keysFoundCopy,
		keysFoundBitRepresentation: kc.keysFoundBitRepresentation,
	}
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
