package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strings"
)

func main() {
	input := io.ReadData("13")
	groups := strings.Split(input, "\r\n\r\n")

	sum := 0
	for i, group := range groups {
		parts := strings.Split(group, "\r\n")
		first, _ := createElement(parts[0])
		second, _ := createElement(parts[1])
		compare := first.compare(second)
		if compare == 1 {
			fmt.Printf("Index: %d\n", i+1)
			sum += i + 1
		}
	}
	fmt.Printf("Part 1: %d\n", sum)
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
		c.value = int(elm - '0')
		if line[i] == '0' {
			c.value = 10
			i++
		}
		break
	}
	return c, i
}

func (e element) compare(other element) int {
	if e.isValue && other.isValue {
		if e.value < other.value {
			return 1
		} else if e.value == other.value {
			return 0
		} else {
			return -1
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
			return 1
		}
		if len(e.children) > len(other.children) {
			return -1
		}
		return 0
	}
	if !e.isValue && other.isValue {
		return e.compare(element{children: []element{other}})
	}
	return element{children: []element{e}}.compare(other)
}
