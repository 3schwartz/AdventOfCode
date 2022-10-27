package main

import (
	"advent/pkg/collections"
	"fmt"
	"math"
	"os"
	"strings"
)

func main() {
	lines := readMap("day24_data")

	initialBugMap, bugmapAsBits := initializeBugMap(lines)
	currentBugMap := initialBugMap.findNextDublicate(bugmapAsBits)

	biodiversity := currentBugMap.findBiodiversity()

	fmt.Printf("Part 1: %d\n", biodiversity)

	initialDebtMap := initializeDebtMap(lines)
	debtMapAfterTime := initialDebtMap.letTimeGo(200)
	bugCount := debtMapAfterTime.getBugCount()

	fmt.Printf("Part 2: %d\n", bugCount)
}

type debtCoord struct {
	debt int
	x    int
	y    int
}

func (dc debtCoord) addCoordinates(other coord) debtCoord {
	return debtCoord{dc.debt, dc.x + other.x, dc.y + other.y}
}

func (dc debtCoord) getNeighbors() []debtCoord {
	toAdd := [4]coord{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	neighbors := make([]debtCoord, 0)
	for _, cNew := range toAdd {
		new := dc.addCoordinates(cNew)
		if new.x == 2 && new.y == 2 {
			switch {
			case dc.y < 2:
				neighbors = append(neighbors,
					debtCoord{dc.debt - 1, 0, 0},
					debtCoord{dc.debt - 1, 1, 0},
					debtCoord{dc.debt - 1, 2, 0},
					debtCoord{dc.debt - 1, 3, 0},
					debtCoord{dc.debt - 1, 4, 0})
			case dc.y > 2:
				neighbors = append(neighbors,
					debtCoord{dc.debt - 1, 0, 4},
					debtCoord{dc.debt - 1, 1, 4},
					debtCoord{dc.debt - 1, 2, 4},
					debtCoord{dc.debt - 1, 3, 4},
					debtCoord{dc.debt - 1, 4, 4})
			case dc.x < 2:
				neighbors = append(neighbors,
					debtCoord{dc.debt - 1, 0, 0},
					debtCoord{dc.debt - 1, 0, 1},
					debtCoord{dc.debt - 1, 0, 2},
					debtCoord{dc.debt - 1, 0, 3},
					debtCoord{dc.debt - 1, 0, 4})
			case dc.x > 2:
				neighbors = append(neighbors,
					debtCoord{dc.debt - 1, 4, 0},
					debtCoord{dc.debt - 1, 4, 1},
					debtCoord{dc.debt - 1, 4, 2},
					debtCoord{dc.debt - 1, 4, 3},
					debtCoord{dc.debt - 1, 4, 4})
			}
			continue
		}
		if new.x < 0 {
			neighbors = append(neighbors,
				debtCoord{dc.debt + 1, 1, 2})
		}
		if new.x > 4 {
			neighbors = append(neighbors,
				debtCoord{dc.debt + 1, 3, 2})
		}
		if new.y < 0 {
			neighbors = append(neighbors,
				debtCoord{dc.debt + 1, 2, 1})
		}
		if new.y > 4 {
			neighbors = append(neighbors,
				debtCoord{dc.debt + 1, 2, 3})
		}
		if new.x >= 0 && new.x < 5 && new.y >= 0 && new.y < 5 {
			neighbors = append(neighbors, new)
		}
	}
	return neighbors
}

type debtMap map[debtCoord]struct{}

func (dm debtMap) getBugCount() int {
	return len(dm)
}

func (dm debtMap) letTimeGo(time int) debtMap {
	currentDebtMap := dm
	for i := 0; i < time; i++ {
		visited := make(map[debtCoord]struct{})
		newDebtMap := make(debtMap)

		queue := collections.CreateQueue[debtCoord]()

		for bug := range currentDebtMap {
			queue.Append(bug)
		}

		for queue.Len() > 0 {
			toEvaluate, ok := queue.TryDequeue()
			if !ok {
				break
			}
			_, visitedBefore := visited[toEvaluate]
			if visitedBefore {
				continue
			}
			visited[toEvaluate] = struct{}{}

			_, isBug := currentDebtMap[toEvaluate]
			neightbors := toEvaluate.getNeighbors()
			var neighborBugs int
			for _, neighbor := range neightbors {
				_, isNeighborBug := currentDebtMap[neighbor]
				if isNeighborBug {
					neighborBugs++
				}
				if _, visitedNeighbor := visited[neighbor]; isBug && !visitedNeighbor {
					queue.Append(neighbor)
				}
			}

			if isBug && neighborBugs != 1 {
				// Bug dies
				continue
			}
			if !isBug && neighborBugs != 1 && neighborBugs != 2 {
				// Empty space but do not create
				continue
			}
			// Bug survice or create
			newDebtMap[toEvaluate] = struct{}{}
		}
		currentDebtMap = newDebtMap
	}
	return currentDebtMap
}

func initializeDebtMap(lines []string) debtMap {
	currentBugMap := make(debtMap)
	for r, line := range lines {
		for c, e := range line {
			if r == 2 && c == 2 {
				continue
			}
			currentCoord := debtCoord{0, r, c}
			if e == '#' {
				currentBugMap[currentCoord] = struct{}{}
			}
		}
	}
	return currentBugMap
}

type coord struct {
	x int
	y int
}

func (c coord) getNeighbors() []coord {
	toAdd := [4]coord{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	neighbors := make([]coord, 0)
	for _, cNew := range toAdd {
		new := c.add(cNew)
		if new.x >= 0 && new.x < 5 && new.y >= 0 && new.y < 5 {
			neighbors = append(neighbors, new)
		}
	}
	return neighbors
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type bugMap map[coord]struct{}

func initializeBugMap(lines []string) (bugMap, int64) {
	currentBugMap := make(bugMap)
	var bit int
	var bugmapAsBits int64
	for r, line := range lines {
		for c, e := range line {
			bit++
			currentCoord := coord{r, c}
			if e == '#' {
				currentBugMap[currentCoord] = struct{}{}
			}
			bugmapAsBits = bugmapAsBits | (1 << bit)
		}
	}
	return currentBugMap, bugmapAsBits
}

func (bm bugMap) findNextDublicate(bugmapAsBits int64) bugMap {
	visited := map[int64]struct{}{}
	visited[bugmapAsBits] = struct{}{}
	currentBugMap := bm

	for {
		var newBugmapAsBit int64
		var bit int
		newBugMap := make(bugMap)
		for i := 0; i < 5; i++ {
			for j := 0; j < 5; j++ {
				bit++
				currentCoord := coord{i, j}
				neightbors := currentCoord.getNeighbors()
				_, ok := currentBugMap[currentCoord]
				length := currentBugMap.neighborsInMapCount(neightbors)
				if ok && length != 1 {
					// Bug dies
					continue
				}
				if !ok && length != 1 && length != 2 {
					// Empty space but do not create
					continue
				}
				// Bug survice or create
				newBugMap[currentCoord] = struct{}{}
				newBugmapAsBit = newBugmapAsBit | (1 << bit)
			}
		}
		currentBugMap = newBugMap
		_, match := visited[newBugmapAsBit]
		if match {
			break
		}
		visited[newBugmapAsBit] = struct{}{}
	}
	return currentBugMap
}

func (bm bugMap) neighborsInMapCount(neighbors []coord) int {
	var sum int
	for _, neighbor := range neighbors {
		_, ok := bm[neighbor]
		if ok {
			sum++
		}
	}
	return sum
}

func (bm bugMap) findBiodiversity() int {
	var sum int
	var pow float64
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			_, ok := bm[coord{i, j}]
			if ok {
				sum += int(math.Pow(2, pow))
			}
			pow++
		}
	}
	return sum
}

func readMap(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s.txt", fileName))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")
	return lines
}
