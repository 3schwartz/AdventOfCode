package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestPart1(t *testing.T) {
	// Arrange
	wireOne, wireTwo := readData()
	minimumCalculator := newMinimumCalculator(wireOne, wireTwo)

	// Act
	minimumDistance := minimumCalculator.getMinimumDistance()

	// Assert
	if diff := cmp.Diff(860, minimumDistance); diff != "" {
		t.Error(diff)
	}
}

func TestPart2(t *testing.T) {
	// Arrange
	wireOne, wireTwo := readData()
	minimumCalculator := newMinimumCalculator(wireOne, wireTwo)

	// Act
	minimumSteps := minimumCalculator.getMinimumSteps()

	// Assert
	if diff := cmp.Diff(9238, minimumSteps); diff != "" {
		t.Error(diff)
	}
}

func Test_newMove(t *testing.T) {
	// Arrange
	direction := "R74"

	// Act
	move := newMove(direction)

	// Assert
	if move.Direction != "R" {
		t.Error("Wrong direction", move.Direction)
	}
	if move.Step != 74 {
		t.Error("Wrong step", move.Step)
	}
}

func Test_minimumCalculator_getMinimumDistance(t *testing.T) {
	data := []struct {
		name         string
		wireOneInput []string
		wireTwoInput []string
		expected     int
	}{
		{
			"long",
			[]string{"R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"},
			[]string{"U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"},
			159,
		},
		{
			"short",
			[]string{"R8", "U5", "L5", "D3"},
			[]string{"U7", "R6", "D4", "L4"},
			6,
		},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Arrange
			minimumCalculator := newMinimumCalculator(d.wireOneInput, d.wireTwoInput)

			// Act
			actual := minimumCalculator.getMinimumDistance()

			// Assert
			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_minimumCalculator_getMinimumSteps(t *testing.T) {
	// Arrange
	wireOneInput := []string{"R8", "U5", "L5", "D3"}
	wireTwoInput := []string{"U7", "R6", "D4", "L4"}
	minimumCalculator := newMinimumCalculator(wireOneInput, wireTwoInput)
	// Act
	actual := minimumCalculator.getMinimumSteps()

	// Assert
	if diff := cmp.Diff(30, actual); diff != "" {
		t.Error(diff)
	}
}
