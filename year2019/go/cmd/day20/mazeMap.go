package main

import (
	"advent/pkg/collections"
	"unicode"
)

type deptElement struct {
	from    string
	IsOuter int
}

type mazeMap struct {
	mazeMap map[coord]string
	xSize   int
	ySize   int
}

func createMazeMap(lines []string) *mazeMap {
	visited := map[coord]struct{}{}
	newMazeMap := mazeMap{
		mazeMap: make(map[coord]string),
		xSize:   len(lines),
		ySize:   len(lines[0]),
	}

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
				newMazeMap.mazeMap[currentCoord] = string(c)
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

			newMazeMap.mazeMap[gatePosition.xy] = gatePosition.name
		}
	}
	return &newMazeMap
}

func (mm *mazeMap) findNodes(mazeCoord coord) map[string]int {
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
			symbol, ok := mm.mazeMap[neighbor]
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

func (mm *mazeMap) findNodesWithDebt(mazeCoord coord) map[deptElement]int {
	nodes := make(map[deptElement]int)
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
			symbol, ok := mm.mazeMap[neighbor]
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
			debt := deptElement{
				from: symbol,
			}
			if neighbor.x <= 3 || mm.xSize-3 <= neighbor.x || neighbor.y <= 3 || mm.ySize-3 <= neighbor.y {
				debt.IsOuter = 1
			}
			nodes[debt] = current.steps + 1 - 1
		}
	}
	return nodes
}

func (mm mazeMap) getNeighbors(linesP *[]string, currentCoord coord) []coord {
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

func (mm mazeMap) findPosition(linesP *[]string, currentCoord coord, neighbors []coord) position {
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
