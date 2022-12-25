package main

import (
	"advent2022/pkg/io"
	"crypto/md5"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"strings"
)

var figures [5][]coord

func init() {
	figures[0] = []coord{{2, 4}, {3, 4}, {4, 4}, {5, 4}}
	figures[1] = []coord{{2, 5}, {3, 5}, {4, 5}, {3, 4}, {3, 6}}
	figures[2] = []coord{{2, 4}, {3, 4}, {4, 4}, {4, 5}, {4, 6}}
	figures[3] = []coord{{2, 4}, {2, 5}, {2, 6}, {2, 7}}
	figures[4] = []coord{{2, 4}, {2, 5}, {3, 4}, {3, 5}}
}

func main() {
	jetPattern := io.ReadData("17")
	maxY := findRockHeight(2022, jetPattern, false)
	fmt.Printf("Part 1: %d\n", maxY)

	maxY = findRockHeight(1_000_000_000_000, jetPattern, false)
	fmt.Printf("Part 2: %d\n", maxY)
}

type coord struct {
	X int
	Y int
}

func (c coord) add(other coord) coord {
	return coord{c.X + other.X, c.Y + other.Y}
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
		move.X = -1
		move.Y = 0
	case '>':
		move.X = 1
		move.Y = 0
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
		if shifted.X < 0 || shifted.X > 6 || shifted.Y == 0 {
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

type state struct {
	shape  int
	jet    int
	latest string
}

func createState(figures map[coord]struct{}, i, jet, maxY int) state {
	latest := make([]coord, 0)
	for c := range figures {
		t := maxY - c.Y
		if t < 250 {
			latest = append(latest, coord{c.X, t})
		}
	}
	sort.SliceStable(latest, func(s1, s2 int) bool {
		if latest[s1].X == latest[s2].X {
			return latest[s1].Y < latest[s2].Y
		}
		return latest[s1].X < latest[s2].X
	})
	f, _ := json.Marshal(latest)
	hash := md5.Sum(f)
	encoded := base64.StdEncoding.EncodeToString(hash[:])
	return state{i, jet, encoded}
}

type stateResult struct {
	position int
	yMax     int
}

func findRockHeight(loop int, jetPattern string, printFigures bool) int {
	jetPattern = strings.TrimSpace(jetPattern)
	maxY := 0
	gasIdx := -1
	figures := map[coord]struct{}{}
	states := map[state]stateResult{}
	doneShift := false
	extra := 0
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
					if c.Y > maxY {
						maxY = c.Y
					}
				}
				if doneShift {
					break
				}
				newState := createState(figures, i%5, gasIdx%len(jetPattern), maxY)
				oldState, ok := states[newState]

				if !ok {
					states[newState] = stateResult{i, maxY}
					break
				}
				cycle := i - oldState.position
				yIncrease := (maxY - oldState.yMax) * ((loop - i) / cycle)
				loop = i + (loop-i)%cycle
				extra += yIncrease
				doneShift = true
				break
			}
			figure = down
		}
		if printFigures {
			print(figures)
		}
	}
	return maxY + extra
}

func print(figures map[coord]struct{}) {
	yMin := math.MaxInt
	yMax := math.MinInt
	for c := range figures {
		if c.Y < yMin {
			yMin = c.Y
		}
		if c.Y > yMax {
			yMax = c.Y
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
