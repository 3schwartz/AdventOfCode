package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	input := io.ReadData("22")
	jungleMap, instructions, yMax, xMax := createMap(input)

	password := jungleMap.followInstructions(instructions, yMax, xMax)

	fmt.Printf("Part 1: %d", password)
}

type coord struct {
	x, y int
}

func (c coord) add(other coord, xMax, yMax int) coord {
	return coord{modulo(c.x+other.x, xMax), modulo(c.y+other.y, yMax)}
}

func modulo(a, mod int) int {
	return (a%mod + mod) % mod
}

func (c coord) getFacing() int {
	if c.x == 1 && c.y == 0 {
		return 0
	}
	if c.x == 0 && c.y == 1 {
		return 1
	}
	if c.x == -1 && c.y == 0 {
		return 2
	}
	return 3
}

func (c coord) rotateRight() coord {
	return coord{-c.y, c.x}
}

func (c coord) rotateLeft() coord {
	return coord{c.y, -c.x}
}

type jungle map[coord]byte

func (j jungle) findStart(maxX, maxY int) coord {
	for i := 0; i < maxY; i++ {
		for z := 0; z < maxX; z++ {
			c := coord{z, i}
			p, ok := j[c]
			if ok && p != ' ' {
				return c
			}
		}
	}
	return coord{}
}

func (j jungle) followInstructions(instructions []string, maxY, maxX int) int {
	position := j.findStart(maxX, maxY)
	direction := coord{1, 0}
	for _, elm := range instructions {
		if elm == "R" {
			direction = direction.rotateRight()
			continue
		}
		if elm == "L" {
			direction = direction.rotateLeft()
			continue
		}
		walk, err := strconv.Atoi(elm)
		if err != nil {
			panic(err)
		}
		for w := 0; w < walk; w++ {
			newPosition := position.add(direction, maxX, maxY)
			q, ok := j[newPosition]
			for !ok || q == ' ' {
				newPosition = newPosition.add(direction, maxX, maxY)
				q, ok = j[newPosition]
			}
			if q == '#' {
				break
			}
			position = newPosition
		}
	}
	return 1_000*(position.y+1) + 4*(position.x+1) +
		direction.getFacing()
}

func createMap(input string) (jungle, []string, int, int) {
	lines := strings.Split(input, "\r\n")
	positions := make(jungle)
	xMax := math.MinInt
	for i := 0; i < len(lines)-2; i++ {
		for j := 0; j < len(lines[i]); j++ {
			positions[coord{j, i}] = lines[i][j]
		}
		if len(lines[i]) > xMax {
			xMax = len(lines[i])
		}
	}

	instructions := make([]string, 0)
	number := make([]rune, 0)
	for _, elm := range lines[len(lines)-1] {
		if unicode.IsNumber(elm) {
			number = append(number, elm)
			continue
		}
		if len(number) > 0 {
			final := 0
			for _, n := range number {
				final *= 10
				final += int(n - '0')
			}
			instructions = append(instructions, fmt.Sprintf("%d", final))
			number = make([]rune, 0)
		}
		instructions = append(instructions, string(elm))
	}
	if len(number) > 0 {
		final := 0
		for _, n := range number {
			final *= 10
			final += int(n - '0')
		}
		instructions = append(instructions, fmt.Sprintf("%d", final))
	}
	return positions, instructions, len(lines) - 2, xMax
}
