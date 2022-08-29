package main

import (
	"encoding/json"
	"testing"
)

var blackhole int

func BenchmarkFuelGivenOreInput(b *testing.B) {
	chemicals := parseChemicals("day14")
	oreInjected := 1_000_000_000_000
	b.Run("Ratio", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackhole = chemicals.getFuelGivenOre(oreInjected)
		}
	})
	b.Run("Optimum", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackhole = chemicals.getFuelGivenOreUsingOptimum(oreInjected)
		}
	})
}

func Test_solution(t *testing.T) {
	// Arrange
	chemicals := parseChemicals("day14")

	// Act
	oreCount, _ := chemicals.getRawCountFrom("FUEL", 1, map[string]int{})
	oreInjected := 1_000_000_000_000
	actualFuelFloor := chemicals.getFuelGivenOre(oreInjected)
	actualFuelFloorOptimum := chemicals.getFuelGivenOreUsingOptimum(oreInjected)

	// Assert
	if oreCount != 261960 {
		t.Errorf("part 1 not correct got: %d", oreCount)
	}
	if actualFuelFloor != 4366186 {
		t.Errorf("part 2 not correct got: %d", actualFuelFloor)
	}
	if actualFuelFloorOptimum != 4366186 {
		t.Errorf("part 2 using optimum not correct got: %d", actualFuelFloor)
	}
}

func Test_correct_read(t *testing.T) {
	// Arrange & Act
	chemicals := parseChemicals("day14_test")

	// Assert
	if len(*chemicals) != 6 {
		s, _ := json.MarshalIndent(chemicals, "", "\t")
		t.Errorf("chemicals not constructed correct: %s", string(s))
	}
}

func Test_correct_raw_count(t *testing.T) {
	data := []struct {
		fileName string
		expected int
	}{
		{"day14_test", 31},
		{"day14_test2", 165},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			chemicals := parseChemicals(d.fileName)

			// Act
			rawCount, _ := chemicals.getRawCountFrom("FUEL", 1, map[string]int{})

			// Assert
			if rawCount != d.expected {
				t.Errorf("%s: expected: %d, got: %d", d.fileName, d.expected, rawCount)
			}
		})
	}
}
