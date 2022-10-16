package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	codes := read.ReadData("day21")
	intCoder := coders.ASCIIIntCoder{}

	input := []string{
		"NOT C J",
		"AND D J",
		"NOT A T",
		"OR T J",
		"WALK",
	}
	movement := createInput(input)
	damage := intCoder.ReportDust(codes, movement)

	fmt.Printf("Part 1: %d", damage[len(damage)-1])
}

func createInput(input []string) []int {
	movement := make([]int, 0)
	for _, line := range input {
		for _, elm := range line {
			movement = append(movement, int(elm))
		}
		movement = append(movement, '\n')
	}
	return movement
}
