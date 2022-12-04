package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := readData()
	totalWithin := 0
	totalOverlap := 0
	for _, line := range strings.Split(input, "\r\n") {
		pair := strings.Split(line, ",")
		first := createCleanRange(pair[0])
		second := createCleanRange(pair[1])
		if first.isOtherWithin(second) || second.isOtherWithin(first) {
			totalWithin += 1
		}
		if first.isOverlapping(second) {
			totalOverlap += 1
		}
	}

	fmt.Printf("Part 1: %d\n", totalWithin)
	fmt.Printf("Part 2: %d\n", totalOverlap)
}

func createCleanRange(input string) cleanRange {
	pair := strings.Split(input, "-")
	first, err := strconv.Atoi(pair[0])
	if err != nil {
		panic(err)
	}
	second, err := strconv.Atoi(pair[1])
	if err != nil {
		panic(err)
	}
	return cleanRange{
		from: first,
		to:   second,
	}

}

type cleanRange struct {
	from int
	to   int
}

func (c cleanRange) isOtherWithin(other cleanRange) bool {
	return c.from <= other.from && other.to <= c.to
}

func (c cleanRange) isOverlapping(other cleanRange) bool {
	return !(c.to < other.from || other.to < c.from)
}

func readData() string {
	f, err := os.ReadFile("../../../data/day4_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
