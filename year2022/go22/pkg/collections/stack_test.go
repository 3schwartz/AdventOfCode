package collections

import "testing"

func Test_stack(t *testing.T) {
	// Arrange
	stack := CreateStack[int]()

	// Act
	stack.Append(1)
	stack.Append(2)
	stack.Append(3)
	actual, worked := stack.TryPop()

	// Assert
	if worked != true || actual != 3 {
		t.Errorf("got: %d", actual)
	}
}
