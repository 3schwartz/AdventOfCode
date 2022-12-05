package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData(5)

	part1(input)
	part2(input)
}

func part2(input string) {
	sections := strings.Split(input, "\r\n\r\n")
	cargoStack := createInitialStack(sections[0])

	for _, line := range strings.Split(sections[1], "\r\n") {
		lineMove := createMove(line)
		fromQueue := cargoStack[lineMove.from]
		toQueue := cargoStack[lineMove.to]
		temp := make([]byte, lineMove.count)
		for i := lineMove.count - 1; 0 <= i; i-- {
			unit, success := fromQueue.TryPop()
			if !success {
				panic("not enough in stack")
			}
			temp[i] = unit
		}
		for _, t := range temp {
			toQueue.Append(t)
		}
	}

	cargoStack.printTopOfStack()
}

func part1(input string) {
	sections := strings.Split(input, "\r\n\r\n")
	cargoStack := createInitialStack(sections[0])

	for _, line := range strings.Split(sections[1], "\r\n") {
		lineMove := createMove(line)
		fromQueue := cargoStack[lineMove.from]
		toQueue := cargoStack[lineMove.to]
		for i := 0; i < lineMove.count; i++ {
			unit, success := fromQueue.TryPop()
			if !success {
				panic("not enough in stack")
			}
			toQueue.Append(unit)
		}
	}

	cargoStack.printTopOfStack()
}

type move struct {
	count int
	from  int
	to    int
}

func createMove(line string) move {
	instructions := strings.Split(line, " ")
	count, err := strconv.Atoi(instructions[1])
	if err != nil {
		panic(err)
	}
	from, err := strconv.Atoi(instructions[3])
	if err != nil {
		panic(err)
	}
	to, err := strconv.Atoi(instructions[5])
	if err != nil {
		panic(err)
	}
	return move{
		count: count,
		from:  from,
		to:    to,
	}
}

type stack map[int]*collections.Stack[byte]

func createStack() stack {
	return map[int]*collections.Stack[byte]{}
}

func (s stack) printTopOfStack() {
	i := 1
	for {
		q, ok := s[i]
		i++
		if !ok {
			break
		}
		top, success := q.TryPop()
		if !success {
			continue
		}
		fmt.Print(string(top))
		q.Append(top)
	}
	fmt.Print("\n")
}

func createInitialStack(firstSection string) stack {
	first := strings.Split(firstSection, "\r\n")
	crates := createStack()

	lengthFirst := len(first)
	for i := 1; i < len(first[lengthFirst-1]); i += 4 {
		idx := int(first[lengthFirst-1][i] - '0')
		stack := collections.CreateStack[byte]()
		for j := lengthFirst - 2; j >= 0; j-- {
			crate := first[j][i]
			if crate == 32 {
				break
			}
			stack.Append(crate)
		}
		crates[idx] = stack
	}
	return crates
}
