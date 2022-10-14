package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	codes := read.ReadData("day19")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.TractorBeamIntCoder{}
	ch := make(chan coders.BeamPoint)
	go intCoder.FindPointsAffected(intCodes, ch)

	var sum int
	for i := range ch {
		sum += i
	}
	fmt.Printf("Part 1: %d\n", sum)

}
