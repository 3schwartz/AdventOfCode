package main

import "unicode"

type pathGraphFinder struct {
}

func createPathGraphFinder() pathFinder {
	return pathGraphFinder{}
}

func (pf pathGraphFinder) findShortestPath(definition areaDefinition) (int, error) {
	return 0, nil
}

type graph struct {
	graph map[coord]map[rune]int
}

func createGraph(areaMap map[coord]rune) *graph {
	graph := graph{}
	for coord, symbol := range areaMap {
		if !unicode.IsLetter(symbol) {
			continue
		}
		coordNodes := graph.findCoordNodesInGraph(areaMap, coord)
		graph.graph[coord] = coordNodes
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
