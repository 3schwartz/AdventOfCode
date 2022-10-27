package main

import (
	"testing"
)

func Test_testDataPart1(t *testing.T) {
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

func Test_testDataPart2(t *testing.T) {
	// Arrange
	lines := readMap("day24_test_data")

	// Act
	initialDebtMap := initializeDebtMap(lines)
	debtMapAfterTime := initialDebtMap.letTimeGo(10)
	bugCount := debtMapAfterTime.getBugCount()

	// Assert
	if bugCount != 99 {
		t.Error(bugCount)
	}
}
