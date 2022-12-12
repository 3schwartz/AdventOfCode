package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strings"
)

func main() {
	input := io.ReadData("12")
	steps := singleShortestPath(input)

	fmt.Printf("Part 1: %d\n", steps)

	minSteps := multipleShortestPath(input)

	fmt.Printf("Part 2: %d\n", minSteps)
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
			if height == 'S' {
				start = coord{x, y}
				height = 'a'
			}
			areaHeightMap[coord{x, y}] = height
		}
	}
	return areaHeightMap, start
}

func createHeightMapWithStartingPoints(input string) (heightMap, []coord) {
	areaHeightMap := make(heightMap)
	startingPoints := make([]coord, 0)
	for x, line := range strings.Split(input, "\r\n") {
		for y, height := range line {
			if height == 'S' || height == 'a' {
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

func multipleShortestPath(input string) int {
	areaHeightMap, startingPoints := createHeightMapWithStartingPoints(input)
	stepsMin := math.MaxInt
	visited := map[coord]struct{}{}
	for _, start := range startingPoints {
		steps := findShortestPath(areaHeightMap, start, true)
		if steps < stepsMin && steps != 0 {
			stepsMin = steps
		}
		visited[start] = struct{}{}
	}
	return stepsMin
}

func singleShortestPath(input string) int {
	areaHeightMap, start := createHeightMap(input)
	steps := findShortestPath(areaHeightMap, start, false)
	return steps
}

func findShortestPath(areaHeightMap heightMap, start coord, multiple bool) int {
	fmt.Printf("Start: x: %d, y: %d\n", start.x, start.y)
	queue := collections.CreatePriorityMap[coord]()
	queue.Append(start, 0, make(map[coord]struct{}))
	adjacent := areaHeightMap.getAdjacent()
	var visitedFirst bool

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
			if multiple && currentHeight == 'a' && visitedFirst {
				continue
			}
			if currentHeight == 'a' {
				visitedFirst = true
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
