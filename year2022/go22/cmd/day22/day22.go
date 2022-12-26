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

	fmt.Printf("Part 1: %d\n", password)

	password = jungleMap.followCubeInstructions(instructions, yMax, xMax)

	fmt.Printf("Part 2: %d\n", password)
}

type coord struct {
	x, y int
}

func (c coord) addCube(direction coord, xMax, yMax int) (coord, coord) {
	next := coord{c.x + direction.x, c.y + direction.y}
	// A1
	if 0 <= c.x && c.x <= 49 && c.y == 100 && next.y == 99 {
		return direction.rotateRight(), coord{50, c.x%50 + 50}
	}
	// A2
	if 50 <= c.y && c.y <= 99 && c.x == 50 && next.x == 49 {
		return direction.rotateLeft(), coord{c.y % 50, 100}
	}
	// B1
	if 100 <= c.y && c.y <= 149 && c.x == 0 && next.x == -1 {
		return direction.rotateRight().rotateRight(),
			coord{50, 49 - c.y%50}
	}
	// B2
	if 0 <= c.y && c.y <= 49 && c.x == 50 && next.x == 49 {
		return direction.rotateRight().rotateRight(),
			coord{0, 149 - c.y%50}
	}
	// C1
	if 150 <= c.y && c.y <= 199 && c.x == 49 && next.x == 50 {
		return direction.rotateLeft(),
			coord{c.y%50 + 50, 149}
	}
	// C2
	if 50 <= c.x && c.x <= 99 && c.y == 149 && next.y == 150 {
		return direction.rotateRight(),
			coord{49, 150 + c.x%50}
	}
	// D1
	if 150 <= c.y && c.y <= 199 && c.x == 0 && next.x == -1 {
		return direction.rotateLeft(),
			coord{c.y%50 + 50, 0}
	}
	// D2
	if 50 <= c.x && c.x <= 99 && c.y == 0 && next.y == -1 {
		return direction.rotateRight(),
			coord{0, 150 + c.x%50}
	}
	// E1
	if 100 <= c.y && c.y <= 149 && c.x == 99 && next.x == 100 {
		return direction.rotateLeft().rotateLeft(),
			coord{149, 49 - c.y%50}
	}
	// E2
	if 0 <= c.y && c.y <= 49 && c.x == 149 && next.x == 150 {
		return direction.rotateLeft().rotateLeft(),
			coord{99, 149 - c.y%50}
	}
	// F1
	if 100 <= c.x && c.x <= 149 && c.y == 49 && next.y == 50 {
		return direction.rotateRight(),
			coord{99, c.x%50 + 50}
	}
	// F2
	if 50 <= c.y && c.y <= 99 && c.x == 99 && next.x == 100 {
		return direction.rotateLeft(),
			coord{100 + c.y%50, 49}
	}
	// G1
	if 0 <= c.x && c.x <= 49 && c.y == 199 && next.y == 200 {
		return direction,
			coord{100 + c.x%50, 0}
	}
	// G2
	if 100 <= c.x && c.x <= 149 && c.y == 0 && next.y == -1 {
		return direction,
			coord{c.x % 50, 199}
	}

	return direction, coord{modulo(c.x+direction.x, xMax), modulo(c.y+direction.y, yMax)}
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

func (j jungle) followCubeInstructions(instructions []string, maxY, maxX int) int {
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
			newDirection, newPosition := position.addCube(direction, maxX, maxY)
			q, ok := j[newPosition]
			if !ok {
				panic(newPosition)
			}
			if q == '#' {
				break
			}
			direction = newDirection
			position = newPosition
		}
	}
	return 1_000*(position.y+1) + 4*(position.x+1) +
		direction.getFacing()
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
