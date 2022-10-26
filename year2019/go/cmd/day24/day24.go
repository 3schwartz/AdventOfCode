package main

import (
	"fmt"
	"math"
	"os"
	"strings"
)

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

type bitTranslator struct {
	toBitFromCoord map[coord]int
	toCoordFromBit map[int]coord
}

func createBitTranslator() *bitTranslator {
	return &bitTranslator{
		toBitFromCoord: make(map[coord]int),
		toCoordFromBit: make(map[int]coord),
	}
}

func (bt *bitTranslator) addTranslation(c coord, b int) {
	bt.toBitFromCoord[c] = b
	bt.toCoordFromBit[b] = c
}

func main() {
	f, err := os.ReadFile("../../../data/day24_data.txt")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")

	currentBugMap := make(bugMap)
	translator := createBitTranslator()
	visited := map[int64]struct{}{}

	var bit int
	var bugmapAsBits int64
	for r, line := range lines {
		for c, e := range line {
			bit++
			currentCoord := coord{r, c}
			translator.addTranslation(currentCoord, bit)
			if e == '#' {
				currentBugMap[currentCoord] = struct{}{}
			}
			bugmapAsBits = bugmapAsBits | (1 << bit)
		}
	}
	visited[bugmapAsBits] = struct{}{}

	for {
		var newBugmapAsBit int64
		newBugMap := make(bugMap)
		bit = 0
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

	biodiversity := currentBugMap.findBiodiversity()

	fmt.Printf("Part 1: %d", biodiversity)
}
