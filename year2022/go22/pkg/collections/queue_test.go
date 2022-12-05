package collections

import "testing"

func Test_queue(t *testing.T) {
	// Arrange
	queue := CreateQueue[int]()

	// Act
	queue.Append(1)
	queue.Append(2)
	actual, worked := queue.TryDequeue()

	// Assert
	if worked != true || actual != 1 {
		t.Errorf("got: %d", actual)
	}
}
