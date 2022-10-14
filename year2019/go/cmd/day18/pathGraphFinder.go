package main

import (
	"container/heap"
	"errors"
	"fmt"
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

type visitedRobotKey struct {
	first               rune
	second              rune
	third               rune
	fourth              rune
	keysCollectedAsBits int
}

func createVisitedKeyFromAreMap(definition areaDefinition) visitedRobotKey {
	startingPoints := definition.startingPoint.getDiagonals()
	return visitedRobotKey{
		first:               definition.areaMap[startingPoints[0]],
		second:              definition.areaMap[startingPoints[1]],
		third:               definition.areaMap[startingPoints[2]],
		fourth:              definition.areaMap[startingPoints[3]],
		keysCollectedAsBits: 0,
	}
}

func (vrk visitedRobotKey) getKeyFromIndex(index int) (rune, error) {
	switch index {
	case 0:
		return vrk.first, nil
	case 1:
		return vrk.second, nil
	case 2:
		return vrk.third, nil
	case 3:
		return vrk.fourth, nil
	default:
		return 0, fmt.Errorf("index not known: %d", index)
	}
}

func (vrk visitedRobotKey) createVisitedRobotKeyFromIndex(index int, update rune, keys int) (visitedRobotKey, error) {
	goTo := visitedRobotKey{
		first:               vrk.first,
		second:              vrk.second,
		third:               vrk.third,
		fourth:              vrk.fourth,
		keysCollectedAsBits: keys,
	}
	switch index {
	case 0:
		goTo.first = update
	case 1:
		goTo.second = update
	case 2:
		goTo.third = update
	case 3:
		goTo.fourth = update
	default:
		return goTo, fmt.Errorf("index not known: %d", index)
	}
	return goTo, nil
}

type item[T any] struct {
	item  T
	steps int
	index int
}

type priority struct {
	key      rune
	keysBits int
}

func (pf pathGraphFinder) findShortestPathWithRobots(definition areaDefinition) (int, error) {
	graph := createGraph(definition.areaMap)
	visited := map[visitedRobotKey]int{}
	initVisitedKey := createVisitedKeyFromAreMap(definition)
	visited[initVisitedKey] = 0

	priorityQueue := make(map[int]map[visitedRobotKey]struct{}, 1)
	priorityQueue[0] = map[visitedRobotKey]struct{}{
		initVisitedKey: {},
	}

	nodesCache := map[visitedKey]map[rune]int{}

	currentSteps := -1
	for len(priorityQueue) > 0 {
		currentSteps++
		states, ok := priorityQueue[currentSteps]
		if !ok {
			continue
		}
		delete(priorityQueue, currentSteps)
		for currentVisitedKey, _ := range states {
			if definition.keysAsBitRepresentation == currentVisitedKey.keysCollectedAsBits {
				return currentSteps, nil
			}

			if visitedSteps, ok := visited[currentVisitedKey]; ok && visitedSteps < currentSteps {
				continue
			}

			for i := 0; i < 4; i++ {
				key, err := currentVisitedKey.getKeyFromIndex(i)
				if err != nil {
					return 0, err
				}
				cacheKey := visitedKey{key, currentVisitedKey.keysCollectedAsBits}
				nodes, ok := nodesCache[cacheKey]
				if !ok {
					nodes = pf.findEdges(graph, currentVisitedKey.keysCollectedAsBits, key)
					nodesCache[cacheKey] = nodes
				}

				for nextKey, steps := range nodes {
					shift := nextKey - 'a'
					keysBitsNext := currentVisitedKey.keysCollectedAsBits
					keysBitsNext |= 1 << shift
					nextSteps := currentSteps + steps

					nextVisitedKey, _ := currentVisitedKey.createVisitedRobotKeyFromIndex(i, nextKey, keysBitsNext)
					if oldSteps, ok := visited[nextVisitedKey]; ok && oldSteps < nextSteps {
						continue
					}
					visited[nextVisitedKey] = nextSteps

					statesAtStep, ok := priorityQueue[nextSteps]
					if !ok {
						statesAtStep = make(map[visitedRobotKey]struct{}, 1)
					}
					statesAtStep[nextVisitedKey] = struct{}{}
					priorityQueue[nextSteps] = statesAtStep
				}
			}
		}
	}
	return 0, errors.New("not able to find optimal")
}

func (pf pathGraphFinder) findShortestPath(definition areaDefinition) (int, error) {
	graph := createGraph(definition.areaMap)

	visited := map[visitedKey]int{}
	start := definition.areaMap[definition.startingPoint]
	visited[visitedKey{start, 0}] = 0

	priorityQueue := make(graphPriorityQueue[priority], 1)
	priorityQueue[0] = &item[priority]{
		item: priority{
			key: start,
		},
		steps: 0,
		index: 1,
	}
	heap.Init(&priorityQueue)

	nodesCache := map[visitedKey]map[rune]int{}

	for priorityQueue.Len() > 0 {
		itemFromQueue := heap.Pop(&priorityQueue).(*item[priority])
		if definition.keysAsBitRepresentation == itemFromQueue.item.keysBits {
			return itemFromQueue.steps, nil
		}
		currentVisitedKey := visitedKey{itemFromQueue.item.key, itemFromQueue.item.keysBits}
		if visitedSteps, ok := visited[currentVisitedKey]; ok && visitedSteps < itemFromQueue.steps {
			continue
		}

		nodes, ok := nodesCache[currentVisitedKey]
		if !ok {
			nodes = pf.findEdges(graph, itemFromQueue.item.keysBits, itemFromQueue.item.key)
			nodesCache[currentVisitedKey] = nodes
		}
		for nextKey, steps := range nodes {
			shift := nextKey - 'a'
			keysBitsNext := itemFromQueue.item.keysBits
			keysBitsNext |= 1 << shift

			nextSteps := itemFromQueue.steps + steps

			nextVisitedKey := visitedKey{nextKey, keysBitsNext}

			if oldSteps, ok := visited[nextVisitedKey]; ok && oldSteps < nextSteps {
				continue
			}

			visited[nextVisitedKey] = nextSteps

			heap.Push(&priorityQueue, &item[priority]{
				item: priority{
					key:      nextKey,
					keysBits: keysBitsNext,
				},
				steps: nextSteps,
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

	priorityQueue := make(graphPriorityQueue[priority], 1)
	priorityQueue[0] = &item[priority]{
		item: priority{
			key: key,
		},
		steps: 0,
	}

	newKeys := map[rune]struct{}{}
	for priorityQueue.Len() > 0 {
		itemFromQueue := heap.Pop(&priorityQueue).(*item[priority])

		if unicode.IsLower(itemFromQueue.item.key) {
			if containsKey := keys & (1 << (itemFromQueue.item.key - 'a')); containsKey == 0 {
				newKeys[itemFromQueue.item.key] = struct{}{}
				continue
			}
		}

		if visited, ok := optimal[itemFromQueue.item.key]; ok && visited < itemFromQueue.steps {
			continue
		}

		for nextKey, steps := range graph.graph[itemFromQueue.item.key] {
			if unicode.IsUpper(nextKey) {
				if nextContainsKey := keys & (1 << (unicode.ToLower(nextKey) - 'a')); nextContainsKey == 0 {
					continue
				}
			}
			nextSteps := itemFromQueue.steps + steps
			if currentOptimal, ok := optimal[nextKey]; ok && currentOptimal < nextSteps {
				continue
			}

			optimal[nextKey] = nextSteps
			heap.Push(&priorityQueue, &item[priority]{
				item: priority{
					key: nextKey,
				},
				steps: nextSteps,
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
	graph := graph{
		graph: map[rune]map[rune]int{},
	}
	for coord, symbol := range areaMap {
		if !unicode.IsLetter(symbol) && symbol != '@' && !unicode.IsNumber(symbol) {
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
			if _, ok := visited[neighbor]; ok {
				continue
			}
			visited[neighbor] = struct{}{}
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

type graphPriorityQueue[T any] []*item[T]

func (pq graphPriorityQueue[T]) Len() int { return len(pq) }

func (pq graphPriorityQueue[T]) Less(i, j int) bool {
	return pq[i].steps < pq[j].steps
}

func (pq graphPriorityQueue[T]) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *graphPriorityQueue[T]) Push(x any) {
	n := len(*pq)
	item := x.(*item[T])
	item.index = n
	*pq = append(*pq, item)
}

func (pq *graphPriorityQueue[T]) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}
