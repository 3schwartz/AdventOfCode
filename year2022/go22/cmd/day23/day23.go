package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strings"
)

var directions [][]coord

func init() {
	directions = make([][]coord, 4)
	directions[0] = []coord{
		{-1, -1}, {0, -1}, {1, -1}, // NORTH
	}
	directions[1] = []coord{
		{-1, 1}, {0, 1}, {1, 1}, // SOUTH
	}
	directions[2] = []coord{
		{-1, -1}, {-1, 0}, {-1, 1}, // WEST
	}
	directions[3] = []coord{
		{1, -1}, {1, 0}, {1, 1}, // EAST
	}
}

func main() {
	input := io.ReadData("23")

	overview := createOverview(input)
	elves := doRounds(overview, 1)
	empty := elves.findEmpty()

	fmt.Printf("Part 1: %d\n", empty)

	rounds := doToZero(overview)

	fmt.Printf("Part 2: %d\n", rounds)
}

type coord struct {
	x, y int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y}
}

type elves map[coord]struct{}

func (e elves) findEmpty() int {
	xMax := math.MinInt
	yMax := math.MinInt
	xMin := math.MaxInt
	yMin := math.MaxInt
	for c := range e {
		if c.x < xMin {
			xMin = c.x
		}
		if c.y < yMin {
			yMin = c.y
		}
		if c.x > xMax {
			xMax = c.x
		}
		if c.y > yMax {
			yMax = c.y
		}
	}
	empty := 0
	for x := xMin; x <= xMax; x++ {
		for y := yMin; y <= yMax; y++ {
			if _, ok := e[coord{x, y}]; ok {
				continue
			}
			empty++
		}
	}
	return empty
}

func doToZero(elves map[coord]struct{}) int {
	direction := 0
	currentElves := elves
	rounds := 1
	var notMoved int
	for {
		currentElves, notMoved = doRound(currentElves, direction)
		direction = (direction + 1) % 4
		if notMoved == len(currentElves) {
			break
		}
		rounds++
	}
	return rounds
}

func doRounds(elves map[coord]struct{}, rounds int) elves {
	direction := 0
	currentElves := elves
	for i := 0; i < rounds; i++ {
		currentElves, _ = doRound(currentElves, direction)
		direction = (direction + 1) % 4
	}
	return currentElves
}

func doRound(elves map[coord]struct{}, direction int) (map[coord]struct{}, int) {
	movements := map[coord][]coord{}
	notMoved := 0
	for elf := range elves {
		someAround := false
		for _, n := range directions {
			for _, d := range n {
				if _, ok := elves[elf.add(d)]; ok {
					someAround = true
					break
				}
			}
			if someAround {
				break
			}
		}
		if !someAround {
			notMoved++
			continue
		}
		elfDirection := direction
		for {
			noElf := true
			for _, d := range directions[elfDirection%4] {
				if _, ok := elves[elf.add(d)]; ok {
					noElf = false
					break
				}
			}
			if noElf {
				newPosition := elf.add(directions[elfDirection%4][1])
				c, ok := movements[newPosition]
				if !ok {
					c = make([]coord, 0)
				}
				c = append(c, elf)
				movements[newPosition] = c
				break
			}
			elfDirection = (elfDirection + 1) % 4
			if elfDirection == direction {
				break
			}
		}
	}
	newELves := make(map[coord]struct{}, len(elves))
	for elf := range elves {
		newELves[elf] = struct{}{}
	}
	for to, from := range movements {
		if len(from) > 1 {
			continue
		}
		delete(newELves, from[0])

		newELves[to] = struct{}{}
	}
	return newELves, notMoved
}

func createOverview(input string) elves {
	lines := strings.Split(input, "\r\n")

	elves := map[coord]struct{}{}

	for y, line := range lines {
		for x, elm := range line {
			if elm != '#' {
				continue
			}
			elves[coord{x, y}] = struct{}{}
		}
	}
	return elves
}
