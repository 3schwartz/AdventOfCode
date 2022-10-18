package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	lines := readLines("")
	newMazeMap := createMazeMap(lines)
	// newMazeGraph := createMazeGraph(newMazeMap)

	// shortestPath := newMazeGraph.findShortestPathBetweenNodes("AA", "ZZ")

	// fmt.Printf("Part 1: %d\n", shortestPath)

	newDebtMazeGraph := createDebtMazeMap(newMazeMap)

	shortestDebtPath := newDebtMazeGraph.findShortestPathBetweenNodesUsingPriorityMap("AA", "ZZ")

	fmt.Printf("Part 2: %d\n", shortestDebtPath)
}

func readLines(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day20%s_data.txt", fileName))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")
	return lines
}

type pathQueueElement struct {
	from  string
	steps int
}

type queueElement struct {
	coord coord
	steps int
}

type position struct {
	xy   coord
	name string
}

type coord struct {
	x int
	y int
}

func (c coord) getShifts() []coord {
	shifts := [4]coord{
		{-1, 0}, {1, 0}, {0, -1}, {0, 1},
	}
	return shifts[:]
}

func (c coord) getNeighbors() []coord {
	neighbors := make([]coord, 0)
	for _, shift := range c.getShifts() {
		neighbors = append(neighbors, c.add(shift))
	}
	return neighbors
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}
