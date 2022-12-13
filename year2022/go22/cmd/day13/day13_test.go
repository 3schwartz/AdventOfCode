package main

import (
	"advent2022/pkg/io"
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
		if compare == 1 {
			sum += i + 1
		}
	}

	// Assert
	if diff := cmp.Diff(sum, 13); diff != "" {
		t.Error(diff)
	}
}
