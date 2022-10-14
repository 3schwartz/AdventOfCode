package main

import (
	"testing"
)

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
