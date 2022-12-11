package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("11")
	monkeys := createMonkeys(input)
	monkeyBusiness := findMonkeyBusiness(monkeys, 20, true)

	fmt.Printf("Part 1: %d\n", monkeyBusiness)

	monkeys = createMonkeys(input)
	monkeyBusiness = findMonkeyBusiness(monkeys, 10_000, false)

	fmt.Printf("Part 2: %d\n", monkeyBusiness)
}

func findLeastCommonMultiple(monkeys []*monkey) int {
	lcm := 1
	for _, m := range monkeys {
		lcm *= m.modulo
	}
	return lcm
}

func findMonkeyBusiness(monkeys []*monkey, times int, worry bool) int {
	lcm := findLeastCommonMultiple(monkeys)
	for i := 0; i < times; i++ {
		for _, monkey := range monkeys {
			for {
				item, ok := monkey.items.TryDequeue()
				if !ok {
					break
				}
				worryLevel := monkey.operation(item)
				if worry {
					worryLevel /= 3
				} else {
					worryLevel %= lcm
				}
				nextMonkey := monkey.sendFunc(worryLevel)
				monkeys[nextMonkey].items.Append(worryLevel)
				monkey.itemsInspected += 1
			}
		}
	}
	sort.Slice(monkeys, func(i, j int) bool {
		return monkeys[i].itemsInspected > monkeys[j].itemsInspected
	})
	monkeyBusiness := monkeys[0].itemsInspected * monkeys[1].itemsInspected
	return monkeyBusiness
}

func createMonkeys(input string) []*monkey {
	monkeys := make([]*monkey, 0)
	sections := strings.Split(input, "\r\n\r\n")
	for _, section := range sections {
		sectionMonkey := createMonkey(section)
		monkeys = append(monkeys, sectionMonkey)
	}
	return monkeys
}

type monkey struct {
	items          *collections.Queue[int]
	operation      func(int) int
	sendFunc       func(int) int
	itemsInspected int
	modulo         int
}

func createMonkey(section string) *monkey {
	lines := strings.Split(section, "\r\n")
	items := createMonkeyQueue(lines[1])
	operation := createMonkeyOperation(lines[2])
	sendFunc, modulo := createSendNext(lines[3:])
	return &monkey{
		items, operation, sendFunc, 0, modulo,
	}
}

func createSendNext(lines []string) (func(item int) int, int) {
	testLine := lines[0]
	trueLine := lines[1]
	falseLine := lines[2]
	divideBy, err := strconv.Atoi(testLine[21:])
	if err != nil {
		panic(err)
	}
	trueSend, err := strconv.Atoi(trueLine[29:])
	if err != nil {
		panic(err)
	}
	falseSend, err := strconv.Atoi(falseLine[30:])
	if err != nil {
		panic(err)
	}
	return func(x int) int {
		if x%divideBy == 0 {
			return trueSend
		}
		return falseSend
	}, divideBy
}

func createMonkeyOperation(line string) func(int) int {
	operation := strings.Split(line[23:], " ")
	useOperationNumber := true
	operationNumber, err := strconv.Atoi(operation[1])
	if err != nil && operation[1] != "old" {
		panic(err)
	}
	if err != nil && operation[1] == "old" {
		useOperationNumber = false
	}
	operationFunc := func() func(int) int {
		switch operation[0] {
		case "+":
			return func(x int) int {
				if useOperationNumber {
					return x + operationNumber
				}
				return x + x
			}
		case "-":
			return func(x int) int {
				if useOperationNumber {
					return x - operationNumber
				}
				return 0
			}
		case "*":
			return func(x int) int {
				if useOperationNumber {
					return x * operationNumber
				}
				return x * x
			}
		default: // "/"
			return func(x int) int {
				if useOperationNumber {
					return x / operationNumber
				}
				return 1
			}
		}
	}()
	return operationFunc
}

func createMonkeyQueue(line string) *collections.Queue[int] {
	items := collections.CreateQueue[int]()
	for _, sItem := range strings.Split(strings.Split(line, "items: ")[1], ", ") {
		item, err := strconv.Atoi(sItem)
		if err != nil {
			panic(err)
		}
		items.Append(item)
	}
	return items
}
