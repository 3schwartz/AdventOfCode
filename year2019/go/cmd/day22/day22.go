package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	lines := parseData("day22_data")

	output := iterateLines(lines, 10007)

	fmt.Printf("Part 1: %d", output[2019])
}

func parseData(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s.txt", fileName))
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(f), "\r\n")

	return lines
}

func iterateLines(lines []string, size int) []int {
	cutSize := len("cut ")
	incSize := len("deal with increment ")
	input := initializeArray(size)

	for _, line := range lines {
		if strings.Contains(line, "cut") {
			cutInput, err := strconv.Atoi(line[cutSize:])
			if err != nil {
				panic(err)
			}
			input = cut(input, cutInput)
			continue
		}
		if strings.Contains(line, "deal with increment") {
			incInput, err := strconv.Atoi(line[incSize:])
			if err != nil {
				panic(err)
			}
			input = increment(input, incInput)
			continue
		}
		if strings.Contains(line, "deal into new stack") {
			input = stack(input)
			continue
		}
		panic(line)
	}
	return input
}

func stack(input []int) []int {
	output := make([]int, len(input))
	shift := len(input) - 1
	for i := 0; i < len(input); i++ {
		output[i] = -1*input[i] + shift
	}
	return output
}

func increment(input []int, inc int) []int {
	length := len(input)
	output := make([]int, length)
	idx := 0
	for i := 0; i < length; i++ {
		output[idx] = input[i]
		idx = (idx + inc) % length
	}
	return output
}

func cut(input []int, cut int) []int {
	length := len(input)
	output := make([]int, length)
	shift := cut
	// if cut < 0 {
	// 	shift += length
	// }
	for i := 0; i < length; i++ {
		// idx := (i + shift) % length
		idx := modulo(i-shift, length)
		output[idx] = input[i]
	}
	return output
}

func initializeArray(size int) []int {
	input := make([]int, size)
	for i := 0; i < size; i++ {
		input[i] = i
	}
	return input
}

func modulo(in int, mod int) int {
	return (in%mod + mod) % mod
}
