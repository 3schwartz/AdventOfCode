package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
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

func findShortestPath(input string) int {
	areaHeightMap, start := createHeightMap(input)

	queue := collections.CreatePriorityMap[coord]()
	queue.Append(start, 0, make(map[coord]struct{}))
	adjacent := areaHeightMap.getAdjacent()

	for queue.Len() > 0 {
		success, priority, item := queue.TryDequeue()
		if !success {
			break
		}
		steps := priority + 1
		for currentCoord, visited := range item {
			currentHeight, ok := areaHeightMap[currentCoord]
			if !ok {
				continue
			}
			if currentHeight == 'S' {
				currentHeight = 'a'
			}
			for _, adj := range adjacent {
				_, hasVisited := visited[currentCoord]
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
				visitedCopy := queue.CopyVisited(visited)
				visitedCopy[currentCoord] = struct{}{}
				queue.Append(neighborCoord, steps, visitedCopy)
			}
		}
	}
	return 0
}
