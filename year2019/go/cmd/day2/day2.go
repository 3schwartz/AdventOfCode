package main

import (
	"advent/pkg/coders"
	"fmt"
	"os"
	"strings"
)

func main() {
	codes := readData()
	intCodes := coders.ParseIntCodes(codes)
	intCoderInstance := coders.IntCoder{}
	codesModified := intCoderInstance.RunWithNounAndVerb(intCodes, 12, 2)
	fmt.Printf("Part 1: %d\n", codesModified[0])

	optima, err := intCoderInstance.FindOptimalOutput(intCodes, 19690720)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 2: %d\n", optima)
}

func readData() []string {
	f, err := os.ReadFile("../../../data/day2_data.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), ",")
}
