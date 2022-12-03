package main

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

func main() {
	input := readData()
	lines := strings.Split(input, "\r\n")
	totalPriority := 0
	for _, line := range lines {
		length := len(line)
		first := line[0 : length/2]
		second := line[length/2 : length]
		common, err := findCommon(first, second)
		if err != nil {
			panic(err)
		}
		priority := getPriority(common)
		totalPriority += priority
	}

	fmt.Printf("Part 1: %d\n", totalPriority)

	totalBadgePriority := 0
	for i := 0; i < len(lines); i += 3 {
		group := lines[i:(i + 3)]
		common, err := findCommonBadge(group)
		if err != nil {
			panic(err)
		}
		priority := getPriority(common)
		totalBadgePriority += int(priority)
	}

	fmt.Printf("Part 2: %d\n", totalBadgePriority)
}

func findCommonBadge(group []string) (rune, error) {
	if len(group) != 3 {
		return 0, errors.New("length should be three")
	}
	for _, f := range group[0] {
		for _, s := range group[1] {
			for _, t := range group[2] {
				if f == s && s == t {
					return f, nil
				}
			}
		}
	}
	return 0, errors.New("not able to find common badge")
}

func getPriority(input rune) int {
	// a - z -> 1 - 26
	// A - Z -> 27 - 52
	// ASCII
	// a - z -> 97 - 122
	// A - Z -> 65 - 90
	if input > 91 {
		// a - z
		return int(input) - 96
	}
	// A - Z
	return int(input) - 38
}

func findCommon(first string, second string) (rune, error) {
	for _, f := range first {
		for _, s := range second {
			if f == s {
				return s, nil
			}
		}
	}
	return 0, errors.New("not able to find common")
}

func readData() string {
	f, err := os.ReadFile("../../../data/day3_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
