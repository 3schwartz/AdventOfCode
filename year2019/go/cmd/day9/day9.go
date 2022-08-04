package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	codes := read.ReadData("day9")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.IntCoder{}
	output := intCoder.RunWithInput(intCodes, 1)
	fmt.Printf("Part 1: %d\n", output)

	output = intCoder.RunWithInput(intCodes, 2)
	fmt.Printf("Part 2: %d\n", output)
}
