package main

import (
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData(7)

	root := createGraph(input)
	sum := root.findSumCountBelow(100_000)

	fmt.Printf("Part 1: %d\n", sum)

	fileSystemSize := 70_000_000
	freeSize := 30_000_000
	size := root.findSizeOffSmallestDirectoryWhichFreesEnough(fileSystemSize, freeSize)
	fmt.Printf("Part 2: %d\n", size)
}

type directory struct {
	name           string
	parent         *directory
	subDirectories map[string]*directory
	files          map[string]int
	totalSum       int
}

func createDirectory(name string, parent *directory) *directory {
	return &directory{
		name:           name,
		parent:         parent,
		subDirectories: make(map[string]*directory),
		files:          make(map[string]int),
		totalSum:       -1,
	}
}

func (d *directory) findSizeOffSmallestDirectoryWhichFreesEnough(systemSize int, freeSize int) int {
	currentFree := systemSize - d.getSum()
	missingFree := freeSize - currentFree
	belowDirectories := map[string]int{}
	d.findBelowFreeSize(belowDirectories, missingFree)

	smallest := math.MaxInt
	for _, size := range belowDirectories {
		if size < smallest {
			smallest = size
		}
	}
	return smallest
}

func (d *directory) findBelowFreeSize(below map[string]int, freeThreshold int) {
	if d.getSum() >= freeThreshold {
		below[d.name] = d.getSum()
	}
	for _, subDirectory := range d.subDirectories {
		subDirectory.findBelowFreeSize(below, freeThreshold)
	}
}

func (d *directory) findSumCountBelow(threshold int) int {
	totalSum := 0
	if d.getSum() < threshold {
		totalSum += d.getSum()
	}
	for _, subDirectory := range d.subDirectories {
		totalSum += subDirectory.findSumCountBelow(threshold)
	}
	return totalSum
}

func (d *directory) getSum() int {
	if d.totalSum == -1 {
		d.totalSum = 0
		for _, size := range d.files {
			d.totalSum += size
		}
		for _, subDirectory := range d.subDirectories {
			d.totalSum += subDirectory.getSum()
		}
	}

	return d.totalSum
}

func (d *directory) addSubdirectory(subDirectory *directory) {
	d.subDirectories[subDirectory.name] = subDirectory
	d.totalSum = -1
}

func createGraph(input string) *directory {
	lines := strings.Split(input, "\r\n")
	current := createDirectory("root", nil)
	for _, line := range lines {
		if line == "$ cd /" {
			for current.parent != nil {
				current = current.parent
			}
			continue
		}
		if line == "$ ls" || line[0:3] == "dir" {
			continue
		}
		if line == "$ cd .." {
			current = current.parent
			continue
		}
		if len(line) > 5 && line[0:5] == "$ cd " {
			subDirectory := createDirectory(line[5:], current)
			current.addSubdirectory(subDirectory)
			current = subDirectory
			continue
		}
		inputFile := strings.Split(line, " ")
		size, err := strconv.Atoi(inputFile[0])
		if err != nil {
			panic(err)
		}
		current.files[inputFile[0]] = size
	}
	for current.parent != nil {
		current = current.parent
	}
	return current
}
