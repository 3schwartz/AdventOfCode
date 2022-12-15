package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("15")
	cave := createCave(input)
	count := cave.findNonPossiblePositionsAtRow(2000000)

	fmt.Printf("Part 1: %d\n", count)

	distressSignal := cave.findDistressSignal(4000000)

	fmt.Printf("Part 2: %d\n", distressSignal)
}

func toFrom(input string) (int, int) {
	split := strings.Split(input, ", ")
	x, _ := strconv.Atoi(split[0][2:])
	y, err := strconv.Atoi(split[1][2:])
	if err != nil {
		panic(err)
	}
	return x, y
}

func manhattanDistance(xF, xT, yF, yT int) int {
	x := xF - xT
	y := yF - yT
	if x < 0 {
		x *= -1
	}
	if y < 0 {
		y *= -1
	}
	return x + y
}

func getMinMax(s1, s2, b1, b2, xMin, xMax, yMin, yMax, manhattan int) (int, int, int, int) {
	if s1-manhattan < xMin {
		xMin = s1 - manhattan
	}
	if b1-manhattan < xMin {
		xMin = b1 - manhattan
	}
	if s1+manhattan > xMax {
		xMax = s1 + manhattan
	}
	if b1+manhattan > xMax {
		xMax = b1 + manhattan
	}
	if s2-manhattan < yMin {
		yMin = s2 - manhattan
	}
	if b2-manhattan < yMin {
		yMin = b2 - manhattan
	}
	if s2+manhattan > yMax {
		yMax = s2 + manhattan
	}
	if b2+manhattan > yMax {
		yMax = b2 + manhattan
	}
	return xMin, xMax, yMin, yMax
}

type coord struct {
	x int
	y int
}

type cave struct {
	xMax    int
	yMax    int
	xMin    int
	yMin    int
	ground  map[coord]int
	visited map[coord]struct{}
}

func createCave(input string) cave {
	xMax := math.MinInt
	yMax := math.MinInt
	xMin := math.MaxInt
	yMin := math.MaxInt
	ground := map[coord]int{}
	visited := map[coord]struct{}{}
	for _, line := range strings.Split(input, "\r\n") {
		split := strings.Split(line[10:], ": closest beacon is at ")
		s1, s2 := toFrom(split[0])
		b1, b2 := toFrom(split[1])
		manhattan := manhattanDistance(s1, b1, s2, b2)
		ground[coord{s1, s2}] = manhattan
		xMin, xMax, yMin, yMax = getMinMax(s1, s2, b1, b2, xMin, xMax, yMin, yMax, manhattan)
		visited[coord{s1, s2}] = struct{}{}
		visited[coord{b1, b2}] = struct{}{}
	}
	return cave{
		xMax:    xMax,
		yMax:    yMax,
		xMin:    xMin,
		yMin:    yMin,
		ground:  ground,
		visited: visited,
	}
}

func (ca cave) findNonPossiblePositionsAtRow(row int) int {
	count := 0
	for x := ca.xMin; x <= ca.xMax; x++ {
		c := coord{x, row}
		_, ok := ca.visited[c]
		if ok {
			continue
		}
		canExists := true
		for key, value := range ca.ground {
			manhattan := manhattanDistance(key.x, c.x, key.y, c.y)
			if manhattan <= value {
				canExists = false
				break
			}
		}
		if !canExists {
			count++
		}
	}
	return count
}

func (ca cave) findDistressSignal(max int) int {
	for key, value := range ca.ground {
		for xShift := 0; xShift <= value+1; xShift++ {
			yShift := value + 1 - xShift
			for _, direction := range [4]coord{{1, 1}, {-1, -1}, {1, -1}, {-1, 1}} {
				x := direction.x*xShift + key.x
				y := direction.y*yShift + key.y
				if y < 0 || y > max || x < 0 || x > max {
					continue
				}
				canExist := true
				for key, value := range ca.ground {
					manhattan := manhattanDistance(key.x, x, key.y, y)
					if manhattan <= value {
						canExist = false
						break
					}
				}
				if canExist {
					return x*4000000 + y
				}
			}
		}
	}

	return 0
}
