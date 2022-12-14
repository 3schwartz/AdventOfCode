package main

import (
	"advent2022/pkg/io"
	"sort"
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_whenCreate_ThenCorrect(t *testing.T) {
	// Arrange
	input := "[[1],[2,3,4]]"

	// Act
	lists, _ := createElement(input)

	// Assert
	if lists.isValue != false {
		t.Error("should be list")
	}
	if lists.children[0].isValue != false && lists.children[0].children[0].value == 1 {
		t.Error(lists.children[0])
	}
	if lists.children[1].isValue != false && lists.children[1].children[0].value == 2 {
		t.Error(lists.children[1])
	}
}

func Test_group(t *testing.T) {
	// Arrange
	one := "[[[4,6,7]],[[1,10,8,[7,2]],10],[10],[[3,7,8,[6,7],[]],8,[4,[5,0,9,1],[6,1,2,6,5]]]]"
	two := "[[[3,[1,8,7,6],[3,8,7,7],[],[8,5]],[[1,8,10],[10,8,9]],8,[[8],9,[],3],[7,9,8,[2,10,7]]],[8,2],[6]]"

	// Act
	first, _ := createElement(one)
	second, _ := createElement(two)
	compare := first.compare(second)

	// Assert
	if compare != 1 {
		t.Error(compare)
	}
}

func Test_whenCreateWithEmpty_ThenCorrect(t *testing.T) {
	// Arrange
	input := "[[],[2,3,4]]"

	// Act
	lists, _ := createElement(input)

	// Assert
	if len(lists.children[0].children) != 0 {
		t.Error(len(lists.children[0].children))
	}
	if lists.isValue != false {
		t.Error("should be list")
	}
	if lists.children[0].isValue != false && lists.children[0].children[0].isValue == false {
		t.Error(lists.children[0])
	}
	if lists.children[1].isValue != false && lists.children[1].children[0].value == 2 {
		t.Error(lists.children[1])
	}
}

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("13_test")
	groups := strings.Split(input, "\r\n\r\n")

	// Act
	packets := make([]element, 0)
	for _, group := range groups {
		parts := strings.Split(group, "\r\n")
		first, _ := createElement(parts[0])
		second, _ := createElement(parts[1])

		packets = append(packets, second)
		packets = append(packets, first)
	}

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

	// Assert
	if diff := cmp.Diff(dividerSum, 140); diff != "" {
		t.Error(diff)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("13_test")
	groups := strings.Split(input, "\r\n\r\n")

	// Act
	sum := 0
	for i, group := range groups {
		parts := strings.Split(group, "\r\n")
		first, _ := createElement(parts[0])
		second, _ := createElement(parts[1])
		compare := first.compare(second)
		if compare == -1 {
			sum += i + 1
		}
	}

	// Assert
	if diff := cmp.Diff(sum, 13); diff != "" {
		t.Error(diff)
	}
}
