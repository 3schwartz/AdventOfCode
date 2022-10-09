package main

import (
	"container/heap"
	"errors"
	"unicode"
)

type coordVisited struct {
	coord     coord
	keysAsBit int
}

type pathPriorityFinder struct{}

func createPathPriorityFinder() pathFinder {
	return pathPriorityFinder{}
}

func (pf pathPriorityFinder) findBitwiseKeyShift(key rune, currentKeys int) int {
	amountToShift := key - 'a'
	keyBit := 1 << amountToShift
	currentKeys |= keyBit
	return currentKeys
}

func (pf pathPriorityFinder) findShortestPath(areaDefinition areaDefinition) (int, error) {
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
			return collector.steps, nil
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
	return 0, errors.New("not able to find path")
}

type keyPriorityCollector struct {
	areaMap                    map[coord]rune
	currentPosition            coord
	steps                      int
	visitedSinceLastKey        map[coord]bool
	keysFoundCount             int
	keysFound                  map[rune]bool
	keysFoundBitRepresentation int
}

func createKeyCollector(areaMap map[coord]rune, startingPosition coord) *keyPriorityCollector {
	return &keyPriorityCollector{
		areaMap:             areaMap,
		currentPosition:     startingPosition,
		visitedSinceLastKey: map[coord]bool{},
	}
}

func (kc *keyPriorityCollector) getSteps() int {
	return kc.steps
}

func (kc *keyPriorityCollector) getNeighbors(visited map[coordVisited]int) []coord {
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

func (kc *keyPriorityCollector) copy(newPosition coord) *keyPriorityCollector {
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

	return &keyPriorityCollector{
		areaMap:                    areaMapCopy,
		currentPosition:            newPosition,
		steps:                      kc.steps + 1,
		visitedSinceLastKey:        visitedCopy,
		keysFoundCount:             kc.keysFoundCount,
		keysFound:                  keysFoundCopy,
		keysFoundBitRepresentation: kc.keysFoundBitRepresentation,
	}
}
