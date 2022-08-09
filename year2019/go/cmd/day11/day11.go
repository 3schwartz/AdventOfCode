package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	part2()
}

func part2() {
	codes := read.ReadData("day11")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.PaintHullIntCoder{}
	visited := map[coders.Coordinate]int{
		coders.NewCoordinate(0, 0): 1,
	}

	intCoder.PaintHullWithInput(intCodes, visited)

	intCoder.OutputHullPaint(visited, func(output []string) { fmt.Println(output) })
}
