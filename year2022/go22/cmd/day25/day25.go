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
	for input != 0 {
		remainder := input % 5
		if remainder != 2 &&
			remainder != 1 &&
			remainder != 0 &&
			remainder != -1 &&
			remainder != -2 {
			remainder -= 5
			input += 5
		}
		switch remainder {
		case 2:
			snafu = append(snafu, '2')
		case 1:
			snafu = append(snafu, '1')
		case 0:
			snafu = append(snafu, '0')
		case -1:
			snafu = append(snafu, '-')
		case -2:
			snafu = append(snafu, '=')
		default:
			panic(remainder)
		}

		input /= 5
	}

	for i, j := 0, len(snafu)-1; i < j; i, j = i+1, j-1 {
		snafu[i], snafu[j] = snafu[j], snafu[i]
	}
	return string(snafu)
}
