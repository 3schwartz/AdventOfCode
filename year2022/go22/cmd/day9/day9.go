package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData(9)

	tailVisitedCount := findTailVisitedCount(input)

	fmt.Printf("Part 1: %d\n", tailVisitedCount)

	largeTailVisitedCount := findTailVisitedLargeRobe(input)

	fmt.Printf("Part 2: %d\n", largeTailVisitedCount.size())
}

type coord2d struct {
	x int
	y int
}

func (c coord2d) add(other coord2d) coord2d {
	return coord2d{c.x + other.x, c.y + other.y}
}

func (c coord2d) subtract(other coord2d) coord2d {
	return coord2d{c.x - other.x, c.y - other.y}
}

func (c coord2d) findMove(other coord2d) coord2d {
	difference := c.subtract(other)
	move := coord2d{difference.x / 2, difference.y / 2}
	return move
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

type tailVisited map[coord2d]struct{}

func (t tailVisited) size() int {
	return len(t)
}

func (t tailVisited) getMoveCount(count string) int {
	moveCount, err := strconv.Atoi(count)
	if err != nil {
		panic(err)
	}
	return moveCount
}

func (t tailVisited) getMovement(move string) coord2d {
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

func findTailVisitedLargeRobe(input string) tailVisited {
	knots := [10]coord2d{}
	for i := 0; i < 10; i++ {
		knots[i] = coord2d{}
	}
	tailVisited := make(tailVisited)
	tailVisited[knots[9]] = struct{}{}
	for _, line := range strings.Split(input, "\r\n") {
		parts := strings.Split(line, " ")

		movement := tailVisited.getMovement(parts[0])
		count := tailVisited.getMoveCount(parts[1])

		for i := 0; i < count; i++ {
			knots[0] = knots[0].add(movement)
			diagonalMove := coord2d{}
			move := movement

			for j := 1; j < 10; j++ {
				if knots[j-1].isNeighbor(knots[j]) {
					break
				}
				if knots[j-1].isInAdjacentLine(knots[j]) {
					move = knots[j-1].findMove(knots[j])
					knots[j] = knots[j].add(move)
					diagonalMove.x = 0
					diagonalMove.y = 0
					continue
				}
				if diagonalMove.x != 0 && diagonalMove.y != 0 {
					knots[j] = knots[j].add(diagonalMove)
					continue
				}
				last := knots[j-1].subtract(move)
				diagonalMove = last.subtract(knots[j])
				knots[j] = last
			}
			tailVisited[knots[9]] = struct{}{}
		}
	}

	return tailVisited
}

func findTailVisitedCount(input string) int {
	last := coord2d{}
	tail := coord2d{}
	tailVisited := make(tailVisited)
	tailVisited[tail] = struct{}{}
	for _, line := range strings.Split(input, "\r\n") {
		parts := strings.Split(line, " ")
		movement := tailVisited.getMovement(parts[0])
		count := tailVisited.getMoveCount(parts[1])
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
