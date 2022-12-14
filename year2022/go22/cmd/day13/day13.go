package main

import (
	"advent2022/pkg/io"
	"fmt"
	"sort"
	"strings"
)

func main() {
	input := io.ReadData("13")
	groups := strings.Split(input, "\r\n\r\n")

	packets := make([]element, 0)
	sum := 0
	for i, group := range groups {
		parts := strings.Split(group, "\r\n")
		first, _ := createElement(parts[0])
		second, _ := createElement(parts[1])

		packets = append(packets, second)
		packets = append(packets, first)

		compare := first.compare(second)
		if compare == -1 {
			sum += i + 1
		}
	}
	fmt.Printf("Part 1: %d\n", sum)

	divFirst, _ := createElement("[[2]]")
	divSecond, _ := createElement("[[6]]")
	packets = append(packets, divFirst)
	packets = append(packets, divSecond)

	sort.Slice(packets, func(i, j int) bool {
		return packets[i].compare(packets[j]) == -1
	})

	dividerSum := 1
	for i, packet := range packets {
		if packet.equals(divFirst) || packet.equals(divSecond) {
			dividerSum *= (1 + i)
		}
	}

	fmt.Printf("Part 2: %d\n", dividerSum)
}

type element struct {
	isValue  bool
	isList   bool
	value    int
	children []element
}

func createElement(line string) (element, int) {
	c := element{children: make([]element, 0)}
	i := 0
	for i < len(line) {
		elm := line[i]
		i++
		if elm == ',' || elm == '[' {
			c.isList = true
			child, iReturn := createElement(line[i:])
			if !child.isValue && !child.isList {
				continue
			}
			c.children = append(c.children, child)
			i += iReturn
			continue
		}
		if elm == ']' {
			break
		}
		c.isValue = true
		value := int(elm - '0')
		for line[i] != ',' && line[i] != ']' {
			value *= 10
			value += int(line[i] - '0')
			i++
		}
		c.value = value
		break
	}
	return c, i
}

func (e element) equals(other element) bool {
	if e.isValue != other.isValue || e.isList != other.isList {
		return false
	}
	if e.isValue && e.value != other.value {
		return false
	}
	if len(e.children) != len(other.children) {
		return false
	}
	for i := 0; i < len(e.children); i++ {
		equals := e.children[i].equals(other.children[i])
		if !equals {
			return false
		}
	}
	return true
}

func (e element) compare(other element) int {
	if e.isValue && other.isValue {
		if e.value < other.value {
			return -1
		} else if e.value == other.value {
			return 0
		} else {
			return 1
		}
	}
	if !e.isValue && !other.isValue {
		idx := 0
		for idx < len(e.children) && idx < len(other.children) {
			childCompare := e.children[idx].compare(other.children[idx])
			if childCompare == 1 {
				return 1
			}
			if childCompare == -1 {
				return -1
			}
			idx++
		}
		if len(e.children) < len(other.children) {
			return -1
		}
		if len(e.children) > len(other.children) {
			return 1
		}
		return 0
	}
	if !e.isValue && other.isValue {
		return e.compare(element{children: []element{other}})
	}
	return element{children: []element{e}}.compare(other)
}
