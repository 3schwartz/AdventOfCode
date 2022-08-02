package main

import (
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	data := readData()
	image := newImage(data)

	// Act
	idx := image.findLayerWithFewestZeroDigits()
	oneCount, twoCount := image.findOneAndTwoDigitsCountInLayer(idx)

	// Assert
	if result := oneCount * twoCount; result != 2684 {
		t.Errorf("wrong result: %d with oneCount: %d and twoCount: %d", result, oneCount, twoCount)
	}
}
