package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"strings"
	"testing"
)

var neighbors [5]coord

func init() {
	neighbors = [5]coord{
		{0, 1}, {0, -1}, {1, 0}, {-1, 0}, {0, 0},
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("24_test")

	// Act
	steps := findShortestSteps(input, 1)

	// Assert
	if steps != 18 {
		t.Error(steps)
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("24")

	// Act
	steps := findShortestSteps(input, 2)

	// Assert
	if steps != 54 {
		t.Error(steps)
	}
}

type coord struct {
	X, Y int
}

func (c coord) add(other coord) coord {
	return coord{c.X + other.X, c.Y + other.Y}
}

func modulo(a, mod int) int {
	return (a%mod + mod) % mod
}

type blizzardDefinition struct {
	start, end coord
	x, y       int
}
type blizzards map[coord][]coord

func (b blizzards) move(x, y int) blizzards {
	newBlizzards := make(blizzards)
	for bl, direction := range b {
		for _, d := range direction {
			new := bl.add(d)
			new.X = modulo(new.X, x)
			new.Y = modulo(new.Y, y)
			c, ok := newBlizzards[new]
			if !ok {
				c = make([]coord, 0)
			}
			c = append(c, d)
			newBlizzards[new] = c
		}
	}
	return newBlizzards
}

func initialize(input string) (blizzards, blizzardDefinition) {
	lines := strings.Split(input, "\r\n")

	blizzardMap := make(blizzards)
	for y, line := range lines[1 : len(lines)-1] {
		for x, elm := range line[1 : len(line)-1] {
			if elm == '.' {
				continue
			}
			blizzard := coord{}
			switch elm {
			case '>':
				blizzard.X = 1
				blizzard.Y = 0
			case '<':
				blizzard.X = -1
				blizzard.Y = 0
			case 'V':
				blizzard.X = 0
				blizzard.Y = 1
			case '^':
				blizzard.X = 0
				blizzard.Y = -1
			}
			blizzardMap[coord{x, y}] = []coord{blizzard}
		}
	}

	start := coord{}
	end := coord{}
	for x, elm := range lines[0][1:] {
		if elm == '.' {
			start.X = x
			start.Y = -1
			break
		}
	}
	for x, elm := range lines[len(lines)-1][1:] {
		if elm == '.' {
			end.X = x
			end.Y = len(lines) - 2
			break
		}
	}
	return blizzardMap, blizzardDefinition{
		x:     len(lines[0]) - 2,
		y:     len(lines) - 2,
		start: start,
		end:   end,
	}
}

type move struct {
	position                    coord
	state                       blizzards
	steps, endCount, startCount int
}

type visitStep struct {
	position                    coord
	steps, endCount, startCount int
}

func findShortestSteps(input string, times int) int {

	blizzardsMap, definition := initialize(input)

	queue := collections.CreateQueue[move]()
	queue.Append(move{definition.start, blizzardsMap, 0, 0, 0})
	visited := map[visitStep]struct{}{}
	blizzardCache := map[int]blizzards{}

	for queue.Len() > 0 {
		if queue.Len()%2_000 == 0 {
			fmt.Println(queue.Len())
		}
		elm, ok := queue.TryDequeue()
		if !ok {
			break
		}
		thisVisit := visitStep{elm.position, elm.steps, elm.startCount, elm.endCount}
		if _, ok := visited[thisVisit]; ok {
			continue
		}
		visited[thisVisit] = struct{}{}
		newSteps := elm.steps + 1
		newBlizzards, ok := blizzardCache[newSteps]
		if !ok {
			newBlizzards = elm.state.move(definition.x, definition.y)
			blizzardCache[newSteps] = newBlizzards
		}
		for _, n := range neighbors {
			possible := elm.position.add(n)
			startCount := elm.startCount
			endCount := elm.endCount
			if possible.X == definition.end.X && possible.Y == definition.end.Y &&
				startCount == endCount {
				endCount++
			}
			if endCount == times {
				return newSteps
			}
			if possible.X == definition.start.X && possible.Y == definition.start.Y &&
				startCount == endCount-1 {
				startCount++
			}

			if (possible.X < 0 || possible.X >= definition.x ||
				possible.Y < 0 || possible.Y >= definition.y) &&
				!(possible.X == definition.start.X && possible.Y == definition.start.Y) &&
				!(possible.X == definition.end.X && possible.Y == definition.end.Y) {
				continue
			}

			if _, in := newBlizzards[possible]; in {
				continue
			}
			queue.Append(move{possible, newBlizzards, newSteps, startCount, endCount})
		}
	}
	return 0
}
