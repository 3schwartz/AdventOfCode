package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	codes := read.ReadData("day17")
	intCoder := coders.ASCIIIntCoder{}
	cameraMap := intCoder.CreateCameraMap(codes)
	scaffoldThreshold := intCoder.FindScaffoldIntersectionsAboveThreshold(cameraMap, 1)

	fmt.Printf("Part 1: %d", scaffoldThreshold)
}
