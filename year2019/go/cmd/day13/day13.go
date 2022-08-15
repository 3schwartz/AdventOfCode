package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	codes := read.ReadData("day13")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.ArcadeIntCoder{}
	state := intCoder.PlayArcade(intCodes)

	fmt.Printf("Part 1: %d\n", state.BlockCount)

	intCodes[0] = 2
	state = intCoder.PlayArcade(intCodes)
	fmt.Printf("Part 2: %d, with count %d\n", state.TotalScore, state.BlockCount)
}
