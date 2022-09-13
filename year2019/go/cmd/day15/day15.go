package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"container/heap"
	"fmt"
)

func main() {
	codes := read.ReadData("day15")
	intCodes := coders.ParseIntCodes(codes)
	found, movementCount := findOxygen(intCodes)
	if !found {
		panic("didn't find solution")
	}
	fmt.Printf("Part 1: %d\n", movementCount)
}

func findOxygen(codesInput []int) (bool, int) {
	oxygenFinder := coders.CreateOxygenFinderIntCoder(codesInput)
	pq := PriorityQueue{&OxygenFinderItem{
		value:    oxygenFinder,
		priority: 0,
		index:    0,
	}}
	walls := map[coders.Coordinate]bool{}
	for pq.Len() > 0 {
		item := heap.Pop(&pq).(*OxygenFinderItem)

		if walls[item.value.GetPosition()] {
			continue
		}

		foundOxygen, finders := item.value.FindOxygen(walls)

		if foundOxygen {
			return true, item.value.GetMovementCount()
		}

		for _, finder := range finders {
			heap.Push(&pq, &OxygenFinderItem{
				value:    finder,
				priority: finder.GetMovementCount(),
			})
		}

	}
	return false, 0
}

// https://pkg.go.dev/container/heap
type OxygenFinderItem struct {
	value    *coders.OxygenFinderIntCoder
	priority int
	index    int
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*OxygenFinderItem

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*OxygenFinderItem)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) update(item *OxygenFinderItem, value *coders.OxygenFinderIntCoder, priority int) {
	item.value = value
	item.priority = priority
	heap.Fix(pq, item.index)
}
