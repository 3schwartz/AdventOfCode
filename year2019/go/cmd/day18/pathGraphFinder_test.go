package main

import "testing"

func Test_testGraphCorrectSteps(t *testing.T) {
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
		{"4",
			"day18_test4",
			136,
		},
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
			keyPathFinder := pathGraphFinder{}

			// Act
			steps, err := keyPathFinder.findShortestPath(areaDefinition)

			// Assert
			if err != nil {
				t.Error(err)
			}
			if steps != d.expected {
				t.Errorf("wrong steps: %d, expected: %d", steps, d.expected)
			}
		})
	}

}
