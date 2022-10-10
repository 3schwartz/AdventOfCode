package main

import (
	"fmt"
	"testing"
)

var blackhole int

func Benchmark_findPath(b *testing.B) {
	data := []struct {
		name       string
		createFunc func() pathFinder
	}{
		{
			"priority",
			createPathPriorityFinder,
		},
		{
			"graph",
			createPathGraphFinder,
		},
	}
	for _, v := range []string{"day18_test1", "day18_test2", "day18_test3", "day18_test5"} {
		lines := createLines(v)
		areaDefinition := createAreaDefinition(lines)
		for _, d := range data {
			pathFinder := d.createFunc()
			b.Run(fmt.Sprintf("%s: %s", d.name, v), func(b *testing.B) {
				for i := 0; i < b.N; i++ {
					blackhole, _ = pathFinder.findShortestPath(areaDefinition)
				}
			})
		}
	}
}
