package main

import (
	"container/heap"
	"fmt"
	"math"
)

func main() {
	graph := createInitialMap()
	// for key, value := range graph {
	// 	for ik, iv := range value {
	// 		fmt.Printf("%d: %d: %d\n", key, ik, iv)
	// 	}
	// }
	foo := 5
	bar := foo | (1 << 1)
	egg := foo | (1 << 2)
	fmt.Printf("%d\n", bar)
	fmt.Printf("%d\n", egg)
	shortestPath := graph.findShortestPath('a', 'g')

	switch shortestPath != 8 {
	case true:
		fmt.Printf("error, found: %d\n", shortestPath)
	case false:
		fmt.Print("Success!\n")
	}

	shortestPathUsingPriority := graph.findShortestUsingPriorityQueue('a', 'g')

	switch shortestPathUsingPriority != 8 {
	case true:
		fmt.Printf("error priority, found: %d\n", shortestPathUsingPriority)
	case false:
		fmt.Print("Success priority!\n")
	}
}

type pathCost struct {
	node      rune
	totalCost int
}

type graph map[rune]map[rune]int

func (g graph) findShortestUsingPriorityQueue(from rune, to rune) int {
	minCost := math.MaxInt32
	queue := make(PriorityQueue[rune], 1)
	queue[0] = &Item[rune]{
		Item:     from,
		Priority: 0,
		Index:    1,
	}
	heap.Init(&queue)
	for queue.Len() > 0 {
		current := heap.Pop(&queue).(*Item[rune])

		if current.Priority > minCost {
			break
		}

		for node, cost := range g[current.Item] {
			newCost := current.Priority + cost
			if node == to && newCost < minCost {
				minCost = newCost
				continue
			}
			heap.Push(&queue, &Item[rune]{
				Item:     node,
				Priority: newCost,
			})
		}
	}
	return minCost
}

func (g graph) findShortestPath(from rune, to rune) int {
	shortestPaths := map[rune]int{}
	queue := CreateQueue[pathCost]()
	queue.Append(pathCost{node: from, totalCost: 0})
	for {
		current, ok := queue.TryDequeue()
		if !ok {
			break
		}

		for node, cost := range g[current.node] {
			newCost := current.totalCost + cost
			nodeCost, ok := shortestPaths[node]
			if ok && newCost > nodeCost {
				continue
			}
			shortestPaths[node] = newCost
			queue.Append(pathCost{node: node, totalCost: newCost})
		}
	}
	return shortestPaths[to]
}

func createInitialMap() graph {
	graph := map[rune]map[rune]int{}

	aNodes := map[rune]int{'c': 1, 'b': 6}
	graph['a'] = aNodes

	cNodes := map[rune]int{'a': 1, 'f': 1}
	graph['c'] = cNodes

	bNodes := map[rune]int{'a': 6, 'd': 1, 'e': 1}
	graph['b'] = bNodes

	dNodes := map[rune]int{'f': 2, 'b': 1, 'g': 8}
	graph['d'] = dNodes

	fNodes := map[rune]int{'c': 1, 'd': 2, 'g': 9}
	graph['f'] = fNodes

	gNodes := map[rune]int{'f': 9, 'd': 8, 'e': 2}
	graph['g'] = gNodes

	eNodes := map[rune]int{'b': 1, 'g': 2}
	graph['e'] = eNodes

	return graph
}

type Item[T any] struct {
	Item     T
	Priority int
	Index    int
}

type PriorityQueue[T any] []*Item[T]

func (pq PriorityQueue[T]) Len() int { return len(pq) }

func (pq PriorityQueue[T]) Less(i, j int) bool {
	return pq[i].Priority < pq[j].Priority
}

func (pq PriorityQueue[T]) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].Index = i
	pq[j].Index = j
}

func (pq *PriorityQueue[T]) Push(x any) {
	n := len(*pq)
	item := x.(*Item[T])
	item.Index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue[T]) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.Index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

type Queue[T any] struct {
	bucket []T
}

func CreateQueue[T any]() *Queue[T] {
	return &Queue[T]{}
}

func (q *Queue[T]) Append(element T) {
	q.bucket = append(q.bucket, element)
}

func (q *Queue[T]) TryDequeue() (T, bool) {
	if len(q.bucket) == 0 {
		var dummy T
		return dummy, false
	}
	value := q.bucket[0]
	var zero T
	q.bucket[0] = zero
	q.bucket = q.bucket[1:]
	return value, true
}

func (q *Queue[T]) Len() int {
	return len(q.bucket)
}
