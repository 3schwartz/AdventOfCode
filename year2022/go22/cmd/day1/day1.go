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
	p := picnic{}
	elves := p.elfCreator(input)
	sums := p.getSortedElfCalorieSums(elves)
	maxSum := sums[0]
	fmt.Printf("Part 1: %d\n", maxSum)

	threeHighest := 0
	for _, i := range sums[0:3] {
		threeHighest += i
	}
	fmt.Printf("Part 2: %d\n", threeHighest)
}

type elf struct {
	calories string
}

func (e elf) getCalorieSum() int {
	currentSum := 0
	calories := strings.Split(e.calories, "\r\n")
	for _, calorie := range calories {
		i, err := strconv.Atoi(calorie)
		if err != nil {
			panic(err)
		}
		currentSum += i
	}
	return currentSum
}

type picnic struct{}

func (p picnic) elfCreator(input string) []elf {
	elvesInput := strings.Split(input, "\r\n\r\n")
	elves := make([]elf, len(elvesInput))
	for i, elfInput := range elvesInput {
		elves[i] = elf{elfInput}
	}
	return elves
}

func (p picnic) getSortedElfCalorieSums(elves []elf) []int {
	sums := make([]int, len(elves))
	for _, elf := range elves {
		currentSum := elf.getCalorieSum()
		sums = append(sums, currentSum)
	}
	sort.SliceStable(sums, func(i, j int) bool {
		return sums[i] > sums[j]
	})
	return sums
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
