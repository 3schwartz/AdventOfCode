package main

import (
	"fmt"
	"testing"
)

var blackhole int

func Benchmark_findPath(b *testing.B) {
	data := []struct {
		name      pathFinder
		queueType func(mazeGraph, string, string) int
	}{
		{
			name:      Queue,
			queueType: mazeGraph.findShortestPathBetweenNodes,
		},
		{
			name:      PriorityQueue,
			queueType: mazeGraph.findShortestPathBetweenNodesUsingPriorityQueue,
		},
		{
			name:      PriorityMapping,
			queueType: mazeGraph.findShortestPathBetweenNodesUsingPriorityMap,
		},
	}
	for _, fileName := range []string{"_test1", "_test2", ""} {
		lines := readLines(fileName)
		newMazeMap := createMazeMap(lines)
		newMazeGraph := createMazeGraph(newMazeMap)
		for _, d := range data {
			b.Run(fmt.Sprintf("%s: %s", d.name.String(), fileName), func(b *testing.B) {
				for i := 0; i < b.N; i++ {
					blackhole = d.queueType(newMazeGraph, "AA", "ZZ")
				}
			})
		}
	}
}
