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
	for i := 0; i < len(input); i++ {
		p := power(5, len(input)-i-1)
		switch e := input[i]; {
		case e == '=':
			number += -2 * p
		case e == '-':
			number += -1 * p
		case e == '1':
			number += p
		case e == '2':
			number += 2 * p
		case e == '0':
			continue
		default:
			panic(e)
		}
	}
	return number
}

func power(x, p int) int {
	result := 1
	for i := 0; i < p; i++ {
		result *= x
	}
	return result
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
