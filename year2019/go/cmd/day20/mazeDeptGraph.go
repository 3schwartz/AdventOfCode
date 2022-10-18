package main

import (
	"advent/pkg/collections"
	"math"
)

type mazeDeptPriorityElement struct {
	deptElement deptElement
	steps       int
}

type mazeDebtGraph map[deptElement]map[deptElement]int

func createDebtMazeMap(inputMazeMap *mazeMap) mazeDebtGraph {
	newMazeGraph := make(mazeDebtGraph)
	for mazeCoord, mazeSymbol := range inputMazeMap.mazeMap {
		if mazeSymbol == "." {
			continue
		}
		coordNodes := inputMazeMap.findNodesWithDebt(mazeCoord)
		if mazeSymbol[1] < mazeSymbol[0] {
			mazeSymbol = string([]byte{mazeSymbol[1], mazeSymbol[0]})
		}
		mazeElement := deptElement{
			from: mazeSymbol,
		}
		if mazeCoord.x <= 3 || inputMazeMap.xSize-3 <= mazeCoord.x || mazeCoord.y <= 3 || inputMazeMap.ySize-3 <= mazeCoord.y {
			mazeElement.IsOuter = 1
		}

		currentNodes, ok := newMazeGraph[mazeElement]
		if !ok {
			newMazeGraph[mazeElement] = coordNodes
			continue
		}
		for node, steps := range coordNodes {
			currentNodes[node] = steps
		}
	}
	return newMazeGraph
}

func (mdg mazeDebtGraph) findShortestPathBetweenNodesUsingPriorityMap(from string, to string) int {
	distance := math.MaxInt32

	queue := collections.CreatePriorityMap[mazeDeptPriorityElement]()
	queue.Append(mazeDeptPriorityElement{
		deptElement: deptElement{
			from:    from,
			IsOuter: 1,
		},
		steps: -1,
	}, 0)

	for queue.Len() > 0 {
		queue.Reset()
		ok, priority, items := queue.TryDequeue()
		if !ok {
			break
		}
		if priority > distance {
			break
		}
		for mazeElement := range items {
			nodes, ok := mdg[mazeElement.deptElement]
			if !ok {
				continue
			}
			for nodePort, nodeSteps := range nodes {
				nextSteps := mazeElement.steps + nodeSteps
				if nextSteps >= distance {
					continue
				}
				if nodePort.from == "AA" {
					continue
				}
				if nodePort.from == "ZZ" && priority == 0 {
					distance = nextSteps
					continue
				}
				if nodePort.from == "ZZ" && priority != 0 {
					continue
				}
				if priority == 0 && nodePort.IsOuter == 1 && nodePort.from != "ZZ" && nodePort.from != "AA" {
					continue
				}
				nodeDebt := priority
				switch nodePort.IsOuter {
				case 1:
					nodeDebt--
				case 0:
					nodeDebt++
				}

				queue.Append(mazeDeptPriorityElement{
					deptElement: deptElement{
						from:    nodePort.from,
						IsOuter: nodePort.IsOuter ^ 1,
					},
					steps: nextSteps,
				}, nodeDebt)
			}
		}
	}

	return distance
}
