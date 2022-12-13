package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strings"
	"testing"
)

type element struct {
	isList   bool
	value    int
	children []element
}

// func createLine(line string) element {
// 	for i, elm := range line {
// 		if elm == '[' {
// 			c.isValue = false
// 			getChild := createElement(line[i+1:])
// 		}
// 	}
// }

func createElement(line string) (element, int) {
	c := element{children: make([]element, 0)}
	i := 0
	for i < len(line) {
		elm := line[i]
		i++
		if elm == '[' {
			c.isList = true
		}
		if elm == ',' || elm == '[' {
			child, iReturn := createElement(line[i:])
			c.children = append(c.children, child)
			i += iReturn
			continue
		}
		// if elm == '[' {
		// 	c.isList = true
		// 	child, iReturn := createElement(line[i:])
		// 	c.children = append(c.children, child)
		// 	i += iReturn
		// 	continue
		// }
		if elm == ']' {
			break
		}
		if c.isList == false {
			c.value = int(elm - '0')
			break
		}
		if c.isList == true {
			c.children = append(c.children, element{value: int(elm - '0')})
		}
	}
	return c, i
}

func Test_whenCreate_ThenCorrect(t *testing.T) {
	// Arrange
	input := "[[1],[2,3,4]]"

	// Act
	lists, _ := createElement(input)

	// Assert
	if lists.isList != true {
		t.Error("should be list")
	}
	if lists.children[0].isList != true && lists.children[0].children[0].value == 1 {
		t.Error(lists.children[0])
	}
	if lists.children[1].isList != true && lists.children[1].children[0].value == 2 {
		t.Error(lists.children[1])
	}

}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("13_test")
	groups := strings.Split(input, "\r\n\r\n")
	for _, group := range groups {
		parts := strings.Split(group, "\r\n")
		first, _ := createElement(parts[0])
		second, _ := createElement(parts[1])
		fmt.Print(first, second)
	}

	// Act

	// Assert

}
