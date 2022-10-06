package main

import (
	"container/heap"
	"fmt"
	"os"
	"strings"
	"testing"
)

func Test_testCorrectSteps(t *testing.T) {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s_data.txt", "day18_test1"))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")

	areaDefinition := createAreaMap(lines)

	keyCollector := createKeyCollector(areaDefinition.areaMap, areaDefinition.startingPoint)
	neighbors := keyCollector.getNeighbors()

	fmt.Println(neighbors)

	pq := make(PriorityQueue, 1)
	pq[0] = &Item{
		value:    keyCollector,
		priority: keyCollector.steps,
		index:    1,
	}

	heap.Init(&pq)

	for pq.Len() > 0 {
		item := heap.Pop(&pq).(*Item)
		collector := item.value
		// Do logic
		// If not ., then clear visited

		neighbors := collector.getNeighbors()
		for _, neighbor := range neighbors {
			copied := collector.copy(neighbor)
			heap.Push(&pq, &Item{
				value:    copied,
				priority: copied.steps,
			})
		}

	}

}
