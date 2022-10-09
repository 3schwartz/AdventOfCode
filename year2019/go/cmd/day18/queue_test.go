package main

import "testing"

func Test_queue(t *testing.T) {
	// Arrange
	queue := createQueue[int]()

	// Act
	queue.append(1)
	queue.append(2)
	actual, worked := queue.tryDequeue()

	// Assert
	if worked != true || actual != 1 {
		t.Errorf("got: %d", actual)
	}
}
