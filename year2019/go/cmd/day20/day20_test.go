package main

import (
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
