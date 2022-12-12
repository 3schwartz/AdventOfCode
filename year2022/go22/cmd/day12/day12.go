package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"container/heap"
	"fmt"
	"strings"
)

func main() {
	input := io.ReadData("12")
	steps := findShortestPath(input)

	fmt.Printf("Part 1: %d\n", steps)
}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type heightMap map[coord]rune

func createHeightMap(input string) (heightMap, coord) {
	areaHeightMap := make(heightMap)
	var start coord
	for x, line := range strings.Split(input, "\r\n") {
		for y, height := range line {
			areaHeightMap[coord{x, y}] = height
			if height == 'S' {
				start = coord{x, y}
			}
		}
	}
	return areaHeightMap, start
}

func (h heightMap) getAdjacent() [4]coord {
	return [4]coord{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
}

type state struct {
	position coord
	visited  map[coord]struct{}
}

func (s state) copy(newPosition coord, oldVisited coord) state {
	visited := make(map[coord]struct{})
	for v := range s.visited {
		visited[v] = struct{}{}
	}
	visited[oldVisited] = struct{}{}
	return state{position: newPosition, visited: visited}
}

func findShortestPath(input string) int {
	areaHeightMap, start := createHeightMap(input)

	priorityQueue := make(collections.PriorityQueue[state], 1)
	priorityQueue[0] = &collections.Item[state]{
		Item: state{
			position: start,
			visited:  make(map[coord]struct{}),
		},
		Priority: 0,
	}
	adjacent := areaHeightMap.getAdjacent()
	heap.Init(&priorityQueue)

	for priorityQueue.Len() > 0 {
		itemFromQueue := heap.Pop(&priorityQueue).(*collections.Item[state])
		steps := itemFromQueue.Priority + 1
		currentCoord := itemFromQueue.Item.position
		currentHeight, ok := areaHeightMap[currentCoord]
		if !ok {
			continue
		}
		if currentHeight == 'S' {
			currentHeight = 'a'
		}
		for _, adj := range adjacent {
			_, hasVisited := itemFromQueue.Item.visited[currentCoord]
			if hasVisited {
				continue
			}
			var isEnd bool
			neighborCoord := currentCoord.add(adj)
			neighborHeight, ok := areaHeightMap[neighborCoord]
			if !ok {
				continue
			}
			if neighborHeight == 'E' {
				neighborHeight = 'z'
				isEnd = true
			}
			if currentHeight < neighborHeight-rune(1) {
				continue
			}
			if isEnd {
				return steps
			}
			heap.Push(&priorityQueue, &collections.Item[state]{
				Item:     itemFromQueue.Item.copy(neighborCoord, currentCoord),
				Priority: steps,
			})
		}
	}
	return 0
}
