package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := readData()
	part1(input)
	part2(input)
}

func readData() []string {
	f, err := os.ReadFile("../data/day1_data.txt")
	if err != nil {
		panic(err)
	}
	input := string(f)
	masses := strings.Split(string(input), "\r\n")
	return masses
}

func part2(masses []string) {
	var sum int
	for _, v := range masses {
		mass, err := strconv.Atoi(v)
		if err != nil {
			panic(err)
		}
		for {
			mass = getFuel(mass)
			if mass <= 0 {
				break
			}
			sum += mass
		}
	}
	fmt.Printf("Part 2: %d\n", sum)
}

func part1(masses []string) {
	var sum int
	for _, v := range masses {
		mass, err := strconv.Atoi(v)
		if err != nil {
			panic(err)
		}
		sum += getFuel(mass)
	}

	fmt.Printf("Part 1: %d\n", sum)
}

func getFuel(mass int) int {
	return mass/3 - 2
}
