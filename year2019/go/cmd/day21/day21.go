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

	fmt.Printf("Part 1: %d\n", damage[len(damage)-1])

	inputSecond := []string{
		"NOT C J",
		"NOT B T",
		"OR T J",
		"AND D J",
		"AND H J",
		"NOT A T",
		"OR T J",
		"RUN",
	}
	movementSecond := createInput(inputSecond)

	damageSecond := intCoder.ReportDust(codes, movementSecond)

	fmt.Printf("Part 2: %d\n", damageSecond[len(damageSecond)-1])
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
