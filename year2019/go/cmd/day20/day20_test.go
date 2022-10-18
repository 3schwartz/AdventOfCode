package main

import (
	"fmt"
	"testing"
)

type pathFinder int64

const (
	Queue pathFinder = iota
	PriorityQueue
	PriorityMapping
)

func (pf pathFinder) String() string {
	switch pf {
	case Queue:
		return "Queue"
	case PriorityQueue:
		return "PriorityQueue"
	case PriorityMapping:
		return "PriorityMapping"
	default:
		return "Unknown"
	}
}

func Test_ifelse(t *testing.T) {
	// Arrange
	data := []struct {
		input  int
		expect int
	}{
		{
			input:  1,
			expect: 0,
		},
		{
			input:  0,
			expect: 1,
		},
	}
	for _, d := range data {
		t.Run(fmt.Sprintf("%d", d.input), func(t *testing.T) {
			// Act
			actual := d.input ^ 1

			// Assert
			if actual != d.expect {
				t.Error(actual)
			}
		})
	}
}

func Test_givenDebt_WhenFindPath_ThenCorrect(t *testing.T) {
	// Arrange
	lines := readLines("_test3")
	newMazeMap := createMazeMap(lines)
	newMazeGraph := createDebtMazeMap(newMazeMap)

	// Act
	shortestPath := newMazeGraph.findShortestPathBetweenNodesUsingPriorityMap("AA", "ZZ")

	// Assert
	if shortestPath != 396 {
		t.Errorf("wrong path found: %d", shortestPath)
	}
}

func Test_givenPriorityQueue_WhenTestExamples_ThenCorrect(t *testing.T) {
	data := []struct {
		name      pathFinder
		fileName  string
		expected  int
		queueType func(mazeGraph, string, string) int
	}{
		{
			name:      Queue,
			fileName:  "_test1",
			expected:  23,
			queueType: mazeGraph.findShortestPathBetweenNodes,
		},
		{
			name:      Queue,
			fileName:  "_test2",
			expected:  58,
			queueType: mazeGraph.findShortestPathBetweenNodes,
		},
		{
			name:      PriorityQueue,
			fileName:  "_test1",
			expected:  23,
			queueType: mazeGraph.findShortestPathBetweenNodesUsingPriorityQueue,
		},
		{
			name:      PriorityQueue,
			fileName:  "_test2",
			expected:  58,
			queueType: mazeGraph.findShortestPathBetweenNodesUsingPriorityQueue,
		},
		{
			name:      PriorityMapping,
			fileName:  "_test1",
			expected:  23,
			queueType: mazeGraph.findShortestPathBetweenNodesUsingPriorityMap,
		},
		{
			name:      PriorityMapping,
			fileName:  "_test2",
			expected:  58,
			queueType: mazeGraph.findShortestPathBetweenNodesUsingPriorityMap,
		},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			lines := readLines(d.fileName)
			newMazeMap := createMazeMap(lines)
			newMazeGraph := createMazeGraph(newMazeMap)

			// Act
			shortestPath := d.queueType(newMazeGraph, "AA", "ZZ")

			// Assert
			if shortestPath != d.expected {
				t.Errorf("wrong path found: %d", shortestPath)
			}
		})
	}
}
