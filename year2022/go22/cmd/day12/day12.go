package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"strings"
)

func main() {
	input := io.ReadData("12")
	steps := shortestPath(input, false)

	fmt.Printf("Part 1: %d\n", steps)

	minSteps := shortestPath(input, true)

	fmt.Printf("Part 2: %d\n", minSteps)
}

func shortestPath(input string, multiple bool) int {
	areaHeightMap, start := createHeightMap(input, multiple)
	steps := areaHeightMap.findShortestPath(start)
	return steps
}

type priority struct {
	position coord
	steps    int
}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type heightMap map[coord]rune

func createHeightMap(input string, multiple bool) (heightMap, []coord) {
	areaHeightMap := make(heightMap)
	startingPoints := make([]coord, 0)
	for x, line := range strings.Split(input, "\r\n") {
		for y, height := range line {
			if height == 'S' || multiple && height == 'a' {
				startingPoints = append(startingPoints, coord{x, y})
				height = 'a'
			}
			areaHeightMap[coord{x, y}] = height
		}
	}
	return areaHeightMap, startingPoints
}

func (h heightMap) getAdjacent() [4]coord {
	return [4]coord{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
}

func (h heightMap) findShortestPath(starts []coord) int {
	queue := collections.CreateQueue[priority]()
	visited := make(map[coord]struct{})
	for _, c := range starts {
		queue.Append(priority{position: c, steps: 0})
	}
	adjacent := h.getAdjacent()

	for queue.Len() > 0 {
		item, ok := queue.TryDequeue()
		if !ok {
			break
		}
		_, hasVisited := visited[item.position]
		if hasVisited {
			continue
		}
		visited[item.position] = struct{}{}
		steps := item.steps + 1
		currentCoord := item.position
		currentHeight, ok := h[currentCoord]
		if !ok {
			continue
		}
		for _, adj := range adjacent {
			var isEnd bool
			neighborCoord := currentCoord.add(adj)
			neighborHeight, ok := h[neighborCoord]
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
			queue.Append(priority{position: neighborCoord, steps: steps})
		}
	}
	return 0
}
