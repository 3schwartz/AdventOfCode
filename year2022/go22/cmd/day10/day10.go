package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("10")

	signalStrength := findSignalStrength(input)

	fmt.Printf("Part 1: %d\n", signalStrength)

	fmt.Print("Part 2:\n")
	render(input)
}

func render(input string) {
	lines := strings.Split(input, "\r\n")
	cycle := 0
	crt := 2
	for _, line := range lines {
		cycle++
		draw(crt, cycle)
		if line == "noop" {
			continue
		}
		cycle++
		draw(crt, cycle)
		instruction := getInstruction(line)
		crt = crt + instruction
	}
}

func draw(crt int, position int) {
	shifted := position % 40
	switch shifted == crt || shifted == crt-1 || shifted == crt+1 {
	case true:
		fmt.Print("#")
	case false:
		fmt.Print(".")
	}
	if position%40 == 0 {
		fmt.Print("\n")
	}
}

func findSignalStrength(input string) int {
	lines := strings.Split(input, "\r\n")
	cycle := 0
	X := 1
	signalStrength := 0
	for _, line := range lines {
		cycle++
		signalStrength = updateSignalStrength(X, cycle, signalStrength)
		if line == "noop" {
			continue
		}
		cycle++
		signalStrength = updateSignalStrength(X, cycle, signalStrength)
		instruction := getInstruction(line)
		X = X + instruction
	}
	return signalStrength
}

func getInstruction(line string) int {
	instruction := strings.Split(line, " ")
	toAdd, err := strconv.Atoi(instruction[1])
	if err != nil {
		panic(err)
	}
	return toAdd
}

func updateSignalStrength(X int, cycle int, signalStrength int) int {
	if (cycle-20)%40 == 0 {
		return signalStrength + cycle*X
	}
	return signalStrength
}
