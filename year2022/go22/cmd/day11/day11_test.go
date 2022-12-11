package main

import (
	"advent2022/pkg/io"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_correctMonkeyBusiness(t *testing.T) {
	// Arrange
	input := io.ReadData("11_test")
	monkeys := createMonkeys(input)

	// Act
	monkeyBusiness := findMonkeyBusiness(monkeys, 20, true)

	// Assert
	if diff := cmp.Diff(monkeyBusiness, 10605); diff != "" {
		t.Error(diff)
	}
}

func Test_correctMonkeyBusinessNoWorry(t *testing.T) {
	// Arrange
	input := io.ReadData("11_test")
	monkeys := createMonkeys(input)

	// Act
	monkeyBusiness := findMonkeyBusiness(monkeys, 10_000, false)

	// Assert
	if diff := cmp.Diff(monkeyBusiness, 2713310158); diff != "" {
		t.Error(diff)
	}
}
