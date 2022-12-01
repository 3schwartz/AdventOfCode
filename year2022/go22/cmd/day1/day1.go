package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input := readData()
	elves := strings.Split(input, "\r\n\r\n")
	sums := make([]int, len(elves))
	for _, elf := range elves {
		currentSum := 0
		calories := strings.Split(elf, "\r\n")
		for _, calorie := range calories {
			i, err := strconv.Atoi(calorie)
			if err != nil {
				panic(err)
			}
			currentSum += i
		}
		sums = append(sums, currentSum)
	}
	sort.SliceStable(sums, func(i, j int) bool {
		return sums[i] > sums[j]
	})
	maxSum := sums[0]
	fmt.Printf("Part 1: %d\n", maxSum)

	threeHighest := 0
	for _, i := range sums[0:3] {
		threeHighest += i
	}
	fmt.Printf("Part 2: %d\n", threeHighest)
}

func readData() string {
	path, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	fmt.Println(path)
	f, err := os.ReadFile("../../../data/day1_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
