package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData(9)

	tailVisitedCount := findTailVisitedCount(input)

	fmt.Printf("Part 1: %d", tailVisitedCount)
}

type coord2d struct {
	x int
	y int
}

func (c coord2d) getMoveCount(count string) int {
	moveCount, err := strconv.Atoi(count)
	if err != nil {
		panic(err)
	}
	return moveCount
}

func (c coord2d) getMovement(move string) coord2d {
	switch move {
	case "R":
		return coord2d{1, 0}
	case "L":
		return coord2d{-1, 0}
	case "U":
		return coord2d{0, 1}
	default: // D
		return coord2d{0, -1}
	}
}

func (c coord2d) add(other coord2d) coord2d {
	return coord2d{c.x + other.x, c.y + other.y}
}

func (c coord2d) subtract(other coord2d) coord2d {
	return coord2d{c.x - other.x, c.y - other.y}
}

func (c coord2d) isNeighbor(other coord2d) bool {
	neighbors := [9]coord2d{{0, 0}, {0, 1}, {0, -1}, {1, 0}, {-1, 0},
		{-1, -1}, {1, 1}, {-1, 1}, {1, -1}}
	for _, neighbor := range neighbors {
		shift := c.add(neighbor)
		if shift == other {
			return true
		}
	}
	return false
}

func (c coord2d) isInAdjacentLine(other coord2d) bool {
	difference := c.subtract(other)
	return difference.x*difference.y == 0
}

func findTailVisitedLargeRobe(input string) int {
	knots := [10]coord2d{}
	for i := 0; i < 10; i++ {
		knots[i] = coord2d{}
	}
	tailVisited := map[coord2d]struct{}{}
	tailVisited[knots[9]] = struct{}{}
	for _, line := range strings.Split(input, "\r\n") {
		parts := strings.Split(line, " ")

		movement := knots[0].getMovement(parts[0])
		count := knots[0].getMoveCount(parts[1])

		for i := 0; i < count; i++ {
			knots[0] = knots[0].add(movement)
			diagonalMove := coord2d{}

			for j := 1; j < 10; j++ {
				if knots[j-1].isNeighbor(knots[j]) {
					break
				}
				if diagonalMove.x != 0 && diagonalMove.y != 0 {
					knots[j] = knots[j].add(diagonalMove)
					continue
				}
				if knots[j-1].isInAdjacentLine(knots[j]) {
					knots[j] = knots[j].add(movement)
					continue
				}
				last := knots[j-1].subtract(movement)
				diagonalMove = last.subtract(knots[j])
				knots[j] = last
			}
			tailVisited[knots[9]] = struct{}{}
		}
	}

	print(tailVisited)

	return len(tailVisited)
}

func print(tails map[coord2d]struct{}) {
	rotated := map[coord2d]struct{}{}
	for key := range tails {
		rotated[coord2d{-key.y, key.x}] = struct{}{}

	}
	minX := math.MaxInt
	minY := math.MaxInt
	maxX := math.MinInt
	maxY := math.MinInt
	for c := range rotated {
		if c.x > maxX {
			maxX = c.x
		}
		if c.x < minX {
			minX = c.x
		}
		if c.y > maxY {
			maxY = c.y
		}
		if c.y < minY {
			minY = c.y
		}
	}
	for x := minX; x <= maxX; x++ {
		for y := minY; y <= maxY; y++ {
			_, ok := rotated[coord2d{x, y}]
			if ok {
				fmt.Print("#")
				continue
			}
			fmt.Print(".")
		}
		fmt.Print("\n")
	}
}

func findTailVisitedCount(input string) int {
	last := coord2d{}
	tail := coord2d{}
	tailVisited := map[coord2d]struct{}{}
	tailVisited[tail] = struct{}{}
	for _, line := range strings.Split(input, "\r\n") {
		parts := strings.Split(line, " ")
		movement := last.getMovement(parts[0])
		count := last.getMoveCount(parts[1])
		for i := 0; i < count; i++ {
			move := last.add(movement)
			if !move.isNeighbor(tail) {
				tail = last
				tailVisited[tail] = struct{}{}
			}
			last = move
		}
	}
	return len(tailVisited)
}
