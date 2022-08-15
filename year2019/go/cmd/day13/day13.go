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
	output := intCoder.PlayArcade(intCodes, 2)

	blockTotal := 0
	for _, value := range output {
		if value == 2 {
			blockTotal++
		}
	}
	fmt.Printf("Part 1: %d", blockTotal)
}
