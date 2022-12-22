package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("21")
	monkeys := createMonkeyTree(input)

	sum := monkeys.findSumFrom("root")

	fmt.Printf("Part 1: %d\n", sum)

	yell := monkeys.findCorrectInitial("root", "humn")
	fmt.Printf("Part 2: %d\n", yell)
}

type monkey struct {
	value               int
	action, left, right string
}

func createActionMonkey(left, action, right string) monkey {
	return monkey{0, action, left, right}
}

func createValueMonkey(valueString string) monkey {
	value, err := strconv.Atoi(valueString)
	if err != nil {
		panic(err)
	}
	return monkey{value: value}
}

func (m monkey) resolve(equals, side int) int {
	switch m.action {
	case "+":
		return equals - side
	case "-":
		return equals + side
	case "*":
		return equals / side
	default: // /
		return equals * side
	}
}

type monkeyTree map[string]monkey

func (m monkeyTree) findCorrectInitial(root, end string) int {
	monkey := m[root]
	leftContains := m.contains(monkey.left, end)
	if leftContains {
		rightSum := m.findSumFrom(monkey.right)
		return m.findCorrect(monkey.left, end, rightSum)
	}
	leftSum := m.findSumFrom(monkey.left)
	return m.findCorrect(monkey.right, end, leftSum)
}
func (m monkeyTree) findCorrect(root, end string, equals int) int {
	monkey := m[root]
	if monkey.right == end {
		leftSum := m.findSumFrom(monkey.left)
		return monkey.resolve(equals, leftSum)
	}
	if monkey.left == end {
		rightSum := m.findSumFrom(monkey.right)
		return monkey.resolve(equals, rightSum)
	}
	leftContains := m.contains(monkey.left, end)
	if leftContains {
		rightSum := m.findSumFrom(monkey.right)
		toEqual := monkey.resolve(equals, rightSum)
		return m.findCorrect(monkey.left, end, toEqual)
	}
	leftSum := m.findSumFrom(monkey.left)
	toEqual := monkey.resolve(equals, leftSum)
	return m.findCorrect(monkey.right, end, toEqual)
}

func (m monkeyTree) contains(root, end string) bool {
	monkey := m[root]
	if monkey.value != 0 {
		return false
	}
	if monkey.right == end || monkey.left == end {
		return true
	}
	return m.contains(monkey.left, end) || m.contains(monkey.right, end)
}

func (m monkeyTree) findSumFrom(from string) int {
	monkey, ok := m[from]
	if !ok {
		panic(from)
	}
	if monkey.value != 0 {
		return monkey.value
	}
	leftSum := m.findSumFrom(monkey.left)
	rightSum := m.findSumFrom(monkey.right)
	switch monkey.action {
	case "+":
		return leftSum + rightSum
	case "-":
		return leftSum - rightSum
	case "*":
		return leftSum * rightSum
	default: // /
		return leftSum / rightSum
	}
}

func createMonkeyTree(input string) monkeyTree {
	monkeys := make(map[string]monkey)
	for _, line := range strings.Split(input, "\r\n") {
		split := strings.Split(line, " ")
		name := split[0][:len(split[0])-1]
		if len(split) > 2 {
			monkeys[name] = createActionMonkey(split[1], split[2], split[3])
			continue
		}
		monkeys[name] = createValueMonkey(split[1])
	}
	return monkeys
}
