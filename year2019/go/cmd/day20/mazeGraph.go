package main

import (
	"advent/pkg/collections"
	"container/heap"
	"math"
)

type mazeGraph map[string]map[string]int

func createMazeGraph(inputMazeMap *mazeMap) mazeGraph {
	newMazeGraph := make(mazeGraph)
	for mazeCoord, mazeSymbol := range inputMazeMap.mazeMap {
		if mazeSymbol == "." {
			continue
		}
		coordNodes := inputMazeMap.findNodes(mazeCoord)
		if mazeSymbol[1] < mazeSymbol[0] {
			mazeSymbol = string([]byte{mazeSymbol[1], mazeSymbol[0]})
		}
		currentNodes, ok := newMazeGraph[mazeSymbol]
		if !ok {
			newMazeGraph[mazeSymbol] = coordNodes
			continue
		}
		for node, steps := range coordNodes {
			currentNodes[node] = steps
		}
	}
	return newMazeGraph
}

func (mg mazeGraph) findShortestPathBetweenNodesUsingPriorityQueue(from string, to string) int {

	distance := math.MaxInt32
	queue := make(collections.PriorityQueue[string], 1)
	queue[0] = &collections.Item[string]{
		Item:     from,
		Priority: 0,
		Index:    1,
	}

	heap.Init(&queue)

	for queue.Len() > 0 {
		item := heap.Pop(&queue).(*collections.Item[string])

		if item.Priority >= distance {
			break
		}

		nodes, ok := mg[item.Item]
		if !ok {
			continue
		}
		for nodePort, nodeSteps := range nodes {
			nextSteps := item.Priority + nodeSteps
			if nextSteps >= distance {
				continue
			}
			if nodePort == "ZZ" {
				distance = nextSteps
				continue
			}

			heap.Push(&queue, &collections.Item[string]{
				Item:     nodePort,
				Priority: nextSteps + 1,
			})
		}
	}

	return distance
}

func (mg mazeGraph) findShortestPathBetweenNodes(from string, to string) int {

	distance := math.MaxInt32
	queue := collections.CreateQueue[pathQueueElement]()
	queue.Append(pathQueueElement{
		from:  from,
		steps: 0})

	for {
		current, hasMoreElements := queue.TryDequeue()
		if !hasMoreElements {
			break
		}
		if current.steps > distance {
			continue
		}
		nodes, ok := mg[current.from]
		if !ok {
			continue
		}
		for nodePort, nodeSteps := range nodes {
			nextSteps := current.steps + nodeSteps
			if nextSteps >= distance {
				continue
			}
			if nodePort == "ZZ" {
				distance = nextSteps
				continue
			}

			queue.Append(pathQueueElement{
				from:  nodePort,
				steps: nextSteps + 1,
			})
		}
	}

	return distance
}

func (mg mazeGraph) findShortestPathBetweenNodesUsingPriorityMap(from string, to string) int {

	distance := math.MaxInt32

	queue := collections.CreatePriorityMap[string]()
	queue.Append(from, 0)

	for queue.Len() > 0 {

		ok, priority, items := queue.TryDequeue()
		if !ok {
			break
		}
		if priority > distance {
			break
		}
		for from, _ := range items {
			nodes, ok := mg[from]
			if !ok {
				continue
			}
			for nodePort, nodeSteps := range nodes {
				nextSteps := priority + nodeSteps
				if nextSteps >= distance {
					continue
				}
				if nodePort == "ZZ" {
					distance = nextSteps
					continue
				}

				queue.Append(nodePort, nextSteps+1)
			}
		}
	}

	return distance
}
