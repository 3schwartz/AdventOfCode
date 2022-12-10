package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strings"
)

func main() {
	input := io.ReadData("8")
	scannedTreeGrid := createTreeGrid(input)
	visibleTrees := scannedTreeGrid.getVisibleTrees()

	fmt.Printf("Part 1: %d\n", visibleTrees)

	coverScore := scannedTreeGrid.getTreeCoverScore()

	fmt.Printf("Part 2: %d\n", coverScore)
}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type treeGrid map[coord]int

func createTreeGrid(input string) treeGrid {
	inputTreeGrid := make(treeGrid)
	lines := strings.Split(input, "\r\n")
	for x, line := range lines {
		for y, tree := range line {
			height := int(tree - '0')
			inputTreeGrid[coord{x, y}] = height
		}
	}
	return inputTreeGrid
}

func (t treeGrid) getTreeCoverScore() int {
	neighbors := []coord{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	maxCover := math.MinInt
	for c, height := range t {
		scores := [4]int{}
		for i, neighbor := range neighbors {
			visibleTrees := 0
			shift := neighbor
			for {
				newShift := c.add(shift)
				otherTree, ok := t[newShift]
				if !ok {
					break
				}
				if otherTree >= height {
					visibleTrees++
					break
				}
				visibleTrees++
				shift = shift.add(neighbor)
			}
			scores[i] = visibleTrees
		}
		totalCover := 1
		for _, score := range scores {
			totalCover *= score
		}
		if totalCover > maxCover {
			maxCover = totalCover
		}
	}

	return maxCover
}

func (t treeGrid) getVisibleTrees() int {
	neighbors := []coord{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	visibleSum := 0
	for c, height := range t {

		for _, neighbor := range neighbors {
			shift := neighbor
			visible := true
			for {
				newShift := c.add(shift)
				otherTree, ok := t[newShift]
				if !ok {
					break
				}
				if otherTree >= height {
					visible = false
					break
				}

				shift = shift.add(neighbor)
			}
			if visible {
				visibleSum++
				break
			}
		}
	}

	return visibleSum
}
