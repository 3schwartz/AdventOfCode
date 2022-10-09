package main

import (
	"testing"
)

func Test_testCorrectSteps(t *testing.T) {
	data := []struct {
		name     string
		fileName string
		expected int
	}{
		{"1",
			"day18_test1",
			8,
		},
		{"2",
			"day18_test2",
			86,
		},
		{"3",
			"day18_test3",
			132,
		},
		// {"4",
		// 	"day18_test4",
		// 	136,
		// },
		{"5",
			"day18_test5",
			81,
		},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Arrange
			lines := createLines(d.fileName)

			areaDefinition := createAreaDefinition(lines)
			keyPathFinder := pathFinder{}

			// Act
			collector, err := keyPathFinder.findShortestPath(areaDefinition)

			// Assert
			if err != nil {
				t.Error(err)
			}
			if collector.steps != d.expected {
				t.Errorf("wrong steps: %d, expected: %d", collector.steps, d.expected)
			}
		})
	}

}

func Test_whenShiftBitFromKey_ThenCorrect(t *testing.T) {
	// Arrange
	data := []struct {
		name        string
		currentKey  rune
		currentKeys int
		expected    int
	}{
		{
			"first",
			'a',
			0,
			1,
		},
		{
			"second",
			'b',
			1,
			3,
		},
		{
			"third",
			'b',
			0,
			2,
		},
	}
	collector := pathFinder{}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			actual := collector.findBitwiseKeyShift(d.currentKey, d.currentKeys)

			// Assert
			if actual != d.expected {
				t.Errorf("wrong output, got: %d, expected: %d", actual, d.expected)
			}

		})
	}
}

func TestShift(t *testing.T) {
	// Arrange
	expected := 3
	keysFound := map[rune]bool{'a': true, 'b': true}
	found := 0

	// Act
	for key, _ := range keysFound {
		amountToShift := key - 'a'
		keyBit := 1 << amountToShift
		found |= keyBit
	}

	// Assert
	if found != expected {
		t.Errorf("wrong output: %d, expected: %d", found, expected)
	}

}
