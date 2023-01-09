package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strings"
)

func main() {
	input := io.ReadData("25")
	sum := findSnafuSum(input)
	snafu := integerToSnafu(sum)

	fmt.Printf("Part 1: %s\n", snafu)
}

func snafuToInteger(input string) int {
	number := 0
	base := 1
	for i := len(input) - 1; i >= 0; i-- {
		switch e := input[i]; {
		case e == '=':
			number += -2 * base
		case e == '-':
			number += -1 * base
		case e == '1':
			number += base
		case e == '2':
			number += 2 * base
		}
		base *= 5
	}
	return number
}

func findSnafuSum(input string) int {
	sum := 0
	for _, line := range strings.Split(input, "\r\n") {
		sum += snafuToInteger(line)
	}
	return sum
}

func integerToSnafu(input int) string {
	snafu := make([]byte, 0)
	lookup := []byte{'=', '-', '0', '1', '2'}
	for input != 0 {
		remainder := input % 5
		if remainder > 2 {
			remainder -= 5
			input += 5
		}
		snafu = append(snafu, lookup[remainder+2])
		input /= 5
	}

	for i, j := 0, len(snafu)-1; i < j; i, j = i+1, j-1 {
		snafu[i], snafu[j] = snafu[j], snafu[i]
	}
	return string(snafu)
}
