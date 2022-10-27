package main

import "testing"

func Test_testData(t *testing.T) {
	// Arrange
	lines := readMap("day24_test_data")

	// Act
	initialBugMap, bugmapAsBits := initializeBugMap(lines)
	currentBugMap := initialBugMap.findNextDublicate(bugmapAsBits)

	biodiversity := currentBugMap.findBiodiversity()

	// Asser
	if biodiversity != 2129920 {
		t.Error(biodiversity)
	}

}
