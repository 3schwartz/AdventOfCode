package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("14")

	cave, yMax := createCave(input)
	sand := getSandCount(cave, yMax)

	fmt.Printf("Part 1: %d\n", sand)
}

func getCoords(in string) (int, int) {
	split := strings.Split(in, ",")
	x, errX := strconv.Atoi(split[0])
	y, errY := strconv.Atoi(split[1])
	if errX != nil || errY != nil {
		panic(in)
	}
	return x, y
}

func getToFrom(f int, s int) (int, int) {
	if s > f {
		return f, s
	}
	return s, f
}

type coord struct {
	x int
	y int
}

func createCave(input string) (map[coord]struct{}, int) {
	lines := strings.Split(input, "\r\n")

	cave := make(map[coord]struct{})
	yMax := math.MinInt
	for _, line := range lines {
		segments := strings.Split(line, " -> ")
		xL, yL := getCoords(segments[0])
		for i := 1; i < len(segments); i++ {
			xTemp, yTemp := getCoords(segments[i])
			xF, xT := getToFrom(xL, xTemp)
			yF, yT := getToFrom(yL, yTemp)
			for x := xF; x <= xT; x++ {
				for y := yF; y <= yT; y++ {
					cave[coord{x, y}] = struct{}{}
				}
			}
			xL = xTemp
			yL = yTemp
			if yT > yMax {
				yMax = yT
			}
		}
	}
	return cave, yMax
}

func getSandCount(cave map[coord]struct{}, yMax int) int {
	sand := 0
	for {
		start := coord{500, 0}
		for {
			if start.y > yMax {
				return sand
			}
			if _, ok := cave[coord{start.x, start.y + 1}]; !ok {
				start = coord{start.x, start.y + 1}
				continue
			}
			if _, ok := cave[coord{start.x - 1, start.y + 1}]; !ok {
				start = coord{start.x - 1, start.y + 1}
				continue
			}
			if _, ok := cave[coord{start.x + 1, start.y + 1}]; !ok {
				start = coord{start.x + 1, start.y + 1}
				continue
			}
			cave[start] = struct{}{}
			break
		}
		sand++
	}
}
