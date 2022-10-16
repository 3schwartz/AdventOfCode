package main

import (
	"testing"
)

func Test_examples(t *testing.T) {
	data := []struct {
		fileName string
		expected int
	}{
		{
			fileName: "_test1",
			expected: 23,
		},
		{
			fileName: "_test2",
			expected: 58,
		},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			lines := readLines(d.fileName)
			newMazeMap := createMazeMap(lines)
			newMazeGraph := createMazeGraph(newMazeMap)

			// Act
			shortestPath := newMazeGraph.findShortestPathBetweenNodes("AA", "ZZ")

			// Assert
			if shortestPath != d.expected {
				t.Errorf("wrong path found: %d", shortestPath)
			}
		})
	}
}
