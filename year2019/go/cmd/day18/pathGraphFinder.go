package main

import (
	"container/heap"
	"errors"
	"math"
	"unicode"
)

type pathGraphFinder struct {
}

func createPathGraphFinder() pathFinder {
	return pathGraphFinder{}
}

type visitedKey struct {
	from                rune
	keysCollectedAsBits int
}

type priority struct {
	steps    int
	key      rune
	index    int
	keysBits int
}

func (pf pathGraphFinder) findShortestPath(definition areaDefinition) (int, error) {
	graph := createGraph(definition.areaMap)

	visited := map[visitedKey]int{}
	start := definition.areaMap[definition.startingPoint]
	visited[visitedKey{start, 0}] = 0

	priorityQueue := make(graphPriorityQueue, 1)
	priorityQueue[0] = &priority{
		steps: 0,
		key:   start,
		index: 1,
	}
	heap.Init(&priorityQueue)

	nodesCache := map[visitedKey]map[rune]int{}

	for priorityQueue.Len() > 0 {
		item := heap.Pop(&priorityQueue).(*priority)
		if definition.keysAsBitRepresentation == item.keysBits {
			return item.steps, nil
		}
		currentVisitedKey := visitedKey{item.key, item.keysBits}
		if visitedSteps, ok := visited[currentVisitedKey]; ok && visitedSteps < item.steps {
			continue
		}

		nodes, ok := nodesCache[currentVisitedKey]
		if !ok {
			nodes = pf.findEdges(graph, item.keysBits, item.key)
			nodesCache[currentVisitedKey] = nodes
		}
		for nextKey, steps := range nodes {
			shift := nextKey - 'a'
			keysBitsNext := item.keysBits
			keysBitsNext |= 1 << shift

			nextSteps := item.steps + steps

			nextVisitedKey := visitedKey{nextKey, keysBitsNext}

			if oldSteps, ok := visited[nextVisitedKey]; ok && oldSteps < nextSteps {
				continue
			}

			visited[nextVisitedKey] = nextSteps

			heap.Push(&priorityQueue, &priority{
				steps:    nextSteps,
				key:      nextKey,
				keysBits: keysBitsNext,
			})
		}
	}

	return 0, errors.New("not able to find optimal")
}

func (pf pathGraphFinder) findEdges(graph *graph, keys int, key rune) map[rune]int {
	optimal := map[rune]int{}
	for key, _ := range graph.graph {
		optimal[key] = math.MaxInt
	}
	optimal[key] = 0

	priorityQueue := make(graphPriorityQueue, 1)
	priorityQueue[0] = &priority{
		steps: 0,
		key:   key,
	}

	newKeys := map[rune]struct{}{}
	for priorityQueue.Len() > 0 {
		item := heap.Pop(&priorityQueue).(*priority)

		if unicode.IsLower(item.key) {
			if containsKey := keys & (1 << (item.key - 'a')); containsKey == 0 {
				newKeys[item.key] = struct{}{}
				continue
			}
		}

		if visited, ok := optimal[item.key]; ok && visited < item.steps {
			continue
		}

		for nextKey, steps := range graph.graph[item.key] {
			if unicode.IsUpper(nextKey) {
				if nextContainsKey := keys & (1 << (unicode.ToLower(nextKey) - 'a')); nextContainsKey == 0 {
					continue
				}
			}
			nextSteps := item.steps + steps
			if currentOptimal, ok := optimal[nextKey]; ok && currentOptimal < nextSteps {
				continue
			}

			optimal[nextKey] = nextSteps
			heap.Push(&priorityQueue, &priority{
				steps: nextSteps,
				key:   nextKey,
			})
		}
	}

	edges := make(map[rune]int, len(newKeys))
	for k, _ := range newKeys {
		edges[k] = optimal[k]
	}
	return edges
}

type graph struct {
	graph map[rune]map[rune]int
}

func createGraph(areaMap map[coord]rune) *graph {
	graph := graph{}
	for coord, symbol := range areaMap {
		if !unicode.IsLetter(symbol) || symbol != '@' {
			continue
		}
		coordNodes := graph.findCoordNodesInGraph(areaMap, coord)
		graph.graph[symbol] = coordNodes
	}
	return &graph
}

type nodeQueueElements struct {
	coord coord
	steps int
}

func (g graph) findCoordNodesInGraph(areaMap map[coord]rune, currentCoord coord) map[rune]int {
	visited := map[coord]struct{}{}
	nodes := map[rune]int{}
	queue := createQueue[nodeQueueElements]()

	queue.append(nodeQueueElements{
		coord: currentCoord,
		steps: 0})

	visited[currentCoord] = struct{}{}

	for {
		current, hasMoreElements := queue.tryDequeue()
		if !hasMoreElements {
			break
		}
		for _, neighbor := range current.coord.getNeighbors() {
			symbol, ok := areaMap[neighbor]
			if !ok {
				continue
			}
			if unicode.IsLetter(symbol) {
				nodes[symbol] = current.steps + 1
				continue
			}
			queue.append(nodeQueueElements{neighbor, current.steps + 1})
		}
	}

	return nodes
}

type graphPriorityQueue []*priority

func (pq graphPriorityQueue) Len() int { return len(pq) }

func (pq graphPriorityQueue) Less(i, j int) bool {
	return pq[i].steps < pq[j].steps
}

func (pq graphPriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *graphPriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*priority)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *graphPriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}
