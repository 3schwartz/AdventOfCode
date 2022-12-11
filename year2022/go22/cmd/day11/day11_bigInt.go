package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"math/big"
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

func findMonkeyBusiness(monkeys []*monkey, times int, worry bool) int64 {
	for i := 0; i < times; i++ {
		for _, monkey := range monkeys {
			for {
				item, ok := monkey.items.TryDequeue()
				if !ok {
					break
				}
				worryLevel := monkey.operation(item)
				if worry {
					worryLevel.Div(worryLevel, big.NewInt(3))
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
	items          *collections.Queue[*big.Int]
	operation      func(*big.Int) *big.Int
	sendFunc       func(*big.Int) int
	itemsInspected int64
}

func createMonkey(section string) *monkey {
	lines := strings.Split(section, "\r\n")
	items := createMonkeyQueue(lines[1])
	operation := createMonkeyOperation(lines[2])
	sendFunc := createSendNext(lines[3:])
	return &monkey{
		items, operation, sendFunc, 0,
	}
}

func createSendNext(lines []string) func(item *big.Int) int {
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
	divideByInt64 := big.NewInt(int64(divideBy))
	var limit big.Int
	return func(x *big.Int) int {
		limit.Mod(x, divideByInt64)
		if limit.Int64() == 0 {
			return trueSend
		}
		return falseSend
	}
}

func createMonkeyOperation(line string) func(*big.Int) *big.Int {
	operation := strings.Split(line[23:], " ")
	useOperationNumber := true
	operationNumber, err := strconv.Atoi(operation[1])
	if err != nil && operation[1] != "old" {
		panic(err)
	}
	if err != nil && operation[1] == "old" {
		useOperationNumber = false
	}
	operationNumberInt64 := big.NewInt(int64(operationNumber))
	operationFunc := func() func(*big.Int) *big.Int {
		switch operation[0] {
		case "+":
			return func(x *big.Int) *big.Int {
				if useOperationNumber {
					return x.Add(x, operationNumberInt64)
				}
				return x.Add(x, x)
			}
		case "-":
			return func(x *big.Int) *big.Int {
				if useOperationNumber {
					return x.Sub(x, operationNumberInt64)
				}
				return x.Sub(x, x)
			}
		case "*":
			return func(x *big.Int) *big.Int {
				if useOperationNumber {
					return x.Mul(x, operationNumberInt64)
				}
				return x.Mul(x, x)
			}
		default: // "/"
			return func(x *big.Int) *big.Int {
				if useOperationNumber {
					return x.Div(x, operationNumberInt64)
				}
				return x.Div(x, x)
			}
		}
	}()
	return operationFunc
}

func createMonkeyQueue(line string) *collections.Queue[*big.Int] {
	items := collections.CreateQueue[*big.Int]()
	for _, sItem := range strings.Split(strings.Split(line, "items: ")[1], ", ") {
		item, err := strconv.Atoi(sItem)
		if err != nil {
			panic(err)
		}
		bigInt := big.NewInt(int64(item))
		items.Append(bigInt)
	}
	return items
}
