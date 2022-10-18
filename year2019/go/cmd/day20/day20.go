package main

import (
	"advent/pkg/collections"
	"container/heap"
	"fmt"
	"math"
	"os"
	"strings"
	"unicode"
)

func main() {
	lines := readLines("")
	newMazeMap := createMazeMap(lines)
	newMazeGraph := createMazeGraph(newMazeMap)

	shortestPath := newMazeGraph.findShortestPathBetweenNodes("AA", "ZZ")

	fmt.Printf("Part 1: %d\n", shortestPath)
}

func readLines(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day20%s_data.txt", fileName))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")
	return lines
}

type pathQueueElement struct {
	from  string
	steps int
}

type queueElement struct {
	coord coord
	steps int
}

type position struct {
	xy   coord
	name string
}

type coord struct {
	x int
	y int
}

func (c coord) getShifts() []coord {
	shifts := [4]coord{
		{-1, 0}, {1, 0}, {0, -1}, {0, 1},
	}
	return shifts[:]
}

func (c coord) getNeighbors() []coord {
	neighbors := make([]coord, 0)
	for _, shift := range c.getShifts() {
		neighbors = append(neighbors, c.add(shift))
	}
	return neighbors
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type mazeGraph map[string]map[string]int

func createMazeGraph(inputMazeMap mazeMap) mazeGraph {
	newMazeGraph := make(mazeGraph)
	for mazeCoord, mazeSymbol := range inputMazeMap {
		if mazeSymbol == "." {
			continue
		}
		coordNodes := newMazeGraph.findNodes(inputMazeMap, mazeCoord)
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

func (mg *mazeGraph) findNodes(inputMazeMap mazeMap, mazeCoord coord) map[string]int {
	nodes := make(map[string]int)
	visited := make(map[coord]struct{})
	queue := collections.CreateQueue[queueElement]()

	queue.Append(queueElement{coord: mazeCoord, steps: 0})

	visited[mazeCoord] = struct{}{}

	for {
		current, hasMoreElements := queue.TryDequeue()
		if !hasMoreElements {
			break
		}
		for _, neighbor := range current.coord.getNeighbors() {
			_, ok := visited[neighbor]
			if ok {
				continue
			}
			visited[neighbor] = struct{}{}
			symbol, ok := inputMazeMap[neighbor]
			if !ok {
				continue
			}
			if symbol == "." {
				queue.Append(queueElement{neighbor, current.steps + 1})
				continue
			}
			if symbol[1] < symbol[0] {
				symbol = string([]byte{symbol[1], symbol[0]})
			}
			nodes[symbol] = current.steps + 1 - 2
		}
	}
	return nodes
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

type mazeMap map[coord]string

func (mm *mazeMap) getNeighbors(linesP *[]string, currentCoord coord) []coord {
	lines := *linesP
	columns := len(lines[0])
	rows := len(lines)
	neighbors := make([]coord, 0)
	shifts := currentCoord.getShifts()

	for _, shift := range shifts {
		xShift := shift.x + currentCoord.x
		yShift := shift.y + currentCoord.y
		if xShift < 0 || xShift >= rows || yShift < 0 || yShift >= columns {
			continue
		}
		neighbors = append(neighbors, currentCoord.add(shift))
	}

	return neighbors
}

func (mm *mazeMap) findPosition(linesP *[]string, currentCoord coord, neighbors []coord) position {
	lines := *linesP
	currentLetter := string(lines[currentCoord.x][currentCoord.y])

	var firstLetter string
	var secondLetter string

	var coordOut coord

	var borderToDot bool
	var neighborLetterCoord coord
	var neighborLetter string
	for _, neighbor := range neighbors {
		symbol := lines[neighbor.x][neighbor.y]
		if symbol == '.' {
			borderToDot = true
		}
		if unicode.IsLetter(rune(symbol)) {
			neighborLetterCoord = coord{neighbor.x, neighbor.y}
			neighborLetter = string(lines[neighbor.x][neighbor.y])
		}
	}
	if borderToDot {
		secondLetter = currentLetter
		firstLetter = neighborLetter
		coordOut = currentCoord
	} else {
		secondLetter = neighborLetter
		firstLetter = currentLetter
		coordOut = neighborLetterCoord
	}

	return position{coordOut, firstLetter + secondLetter}
}

func createMazeMap(lines []string) mazeMap {
	visited := map[coord]struct{}{}
	newMazeMap := mazeMap{}
	for m, line := range lines {
		for n, c := range line {
			if c == ' ' {
				continue
			}
			if c == '#' {
				continue
			}
			currentCoord := coord{m, n}
			if c == '.' {
				newMazeMap[currentCoord] = string(c)
				visited[currentCoord] = struct{}{}
				continue
			}
			_, ok := visited[currentCoord]
			if ok {
				continue
			}

			neighbors := newMazeMap.getNeighbors(&lines, currentCoord)
			gatePosition := newMazeMap.findPosition(&lines, currentCoord, neighbors)
			_, ok = visited[gatePosition.xy]
			if ok {
				continue
			}

			newMazeMap[gatePosition.xy] = gatePosition.name
		}
	}
	return newMazeMap
}
