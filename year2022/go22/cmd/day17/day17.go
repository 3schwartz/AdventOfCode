package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strings"
)

var figures [5][]coord

func init() {
	figures[0] = []coord{{2, 3}, {3, 3}, {4, 3}, {5, 3}}
	figures[1] = []coord{{2, 4}, {3, 4}, {4, 4}, {3, 3}, {3, 5}}
	figures[2] = []coord{{2, 3}, {3, 3}, {4, 3}, {4, 4}, {4, 5}}
	figures[3] = []coord{{2, 3}, {2, 4}, {2, 5}, {2, 6}}
	figures[4] = []coord{{2, 3}, {2, 4}, {3, 3}, {3, 4}}
}

func main() {
	jetPattern := io.ReadData("17")

	maxY := findRockHeight(2022, jetPattern, false)

	fmt.Printf("Part 1: %d\n", maxY)
}

type coord struct {
	x int
	y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type figure []coord

func getFigure(idx int, maxY int) figure {
	figureNew := make(figure, len(figures[idx%5]))
	shift := coord{0, maxY}
	for i, c := range figures[idx%5] {
		figureNew[i] = c.add(shift)
	}
	return figureNew
}

func (f figure) gasShiftFigure(jet byte) figure {
	move := coord{}
	switch jet {
	case '<':
		move.x = -1
		move.y = 0
	case '>':
		move.x = 1
		move.y = 0
	}
	moved := make([]coord, len(f))
	for i, c := range f {
		moved[i] = c.add(move)
	}
	return moved
}

func (f figure) fallDown() figure {
	down := make(figure, len(f))
	downMove := coord{0, -1}
	for i := 0; i < len(f); i++ {
		down[i] = f[i].add(downMove)
	}
	return down
}

func (f figure) canMove(figures map[coord]struct{}) bool {
	canMove := true
	for _, shifted := range f {
		if shifted.x < 0 || shifted.x > 6 || shifted.y < 0 {
			canMove = false
			break
		}
		if _, ok := figures[shifted]; ok {
			canMove = false
			break
		}
	}
	return canMove
}

func findRockHeight(loop int, jetPattern string, printFigures bool) int {
	jetPattern = strings.TrimSpace(jetPattern)
	maxY := 0
	gasIdx := -1
	figures := map[coord]struct{}{}
	for i := 0; i < loop; i++ {
		figure := getFigure(i, maxY)
		for {
			gasIdx++
			jet := jetPattern[gasIdx%len(jetPattern)]
			gasShifted := figure.gasShiftFigure(jet)
			canShift := gasShifted.canMove(figures)
			if canShift {
				figure = gasShifted
			}
			down := figure.fallDown()
			canFallDown := down.canMove(figures)
			if !canFallDown {
				for _, c := range figure {
					figures[c] = struct{}{}
					if c.y+1 > maxY {
						maxY = c.y + 1
					}
				}
				break
			}
			figure = down
			if printFigures {
				print(figures)
			}
		}
	}
	return maxY
}

func print(figures map[coord]struct{}) {
	yMin := math.MaxInt
	yMax := math.MinInt
	for c := range figures {
		if c.y < yMin {
			yMin = c.y
		}
		if c.y > yMax {
			yMax = c.y
		}
	}
	for y := yMax; y >= yMin; y-- {
		for x := 0; x <= 6; x++ {
			if _, ok := figures[coord{x, y}]; ok {
				fmt.Print("#")
				continue
			}
			fmt.Print(".")
		}
		fmt.Println()
	}
	fmt.Println("@@@@@@@")
}
