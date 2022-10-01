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
	scaffoldThreshold := intCoder.FindScaffoldIntersections(cameraMap)

	fmt.Printf("Part 1: %d", scaffoldThreshold)

	intCoder.Print(cameraMap)

	robot, position, err := intCoder.GetRobotPosition(cameraMap)
	if err != nil {
		panic(err)
	}

	movements := intCoder.GetMovements(cameraMap, robot, position)

	fmt.Println("Print movements:")
	for _, movement := range movements {
		fmt.Print(movement)

	}
	fmt.Println()
	fmt.Println("---")

	// TODO: Find patterns
	bar := "R"
	foo := 'R'
	fmt.Println(bar)
	fmt.Println(bar[0])
	fmt.Println(foo)
}
