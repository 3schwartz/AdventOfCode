package coders

import (
	"fmt"
	"testing"

	"github.com/google/go-cmp/cmp"
)

// Post to https://stackoverflow.com/questions/2818852/is-there-a-queue-implementation/73099149#73099149
var blackholeGeneric int
var blackholeInterface int
var blackholeInterfaceBool bool

func Benchmark_queues(b *testing.B) {
	for _, v := range []int{1, 10, 100, 1_000} {
		input := make([]int, v)
		b.Run(fmt.Sprintf("QueueGeneric-Size %d", v), func(b *testing.B) {
			queue := newQueue[int]()
			for i := 0; i < b.N; i++ {
				for _, element := range input {
					queue.append(element)
				}
				for i := 0; i < len(input); i++ {
					blackholeGeneric, _ = queue.tryDequeue()
				}
			}
		})
		b.Run(fmt.Sprintf("QueueGeneric-Append-Size %d", v), func(b *testing.B) {
			queue := newQueue[int]()
			for i := 0; i < b.N; i++ {
				for _, element := range input {
					queue.append(element)
				}
			}
		})
		b.Run(fmt.Sprintf("QueueInterface-Size %d", v), func(b *testing.B) {
			queueInterface := newQueueInterface()
			for i := 0; i < b.N; i++ {
				for _, element := range input {
					queueInterface.append(element)
				}
				for i := 0; i < len(input); i++ {
					blackholeInterfaceBool, _ = queueInterface.tryDequeue(&blackholeInterface)
				}
			}
		})
		b.Run(fmt.Sprintf("QueueInterface-Append-Size %d", v), func(b *testing.B) {
			queueInterface := newQueueInterface()
			for i := 0; i < b.N; i++ {
				for _, element := range input {
					queueInterface.append(element)
				}
			}
		})
	}
}

func Test_queue(t *testing.T) {
	// Arrange
	queue := newQueue[int]()
	queue.append(1)
	queue.append(2)
	queue.append(3)

	// Act
	valueOne, okOne := queue.tryDequeue()
	valueTwo, okTwo := queue.tryDequeue()

	// Assert
	if !okOne || !okTwo {
		t.Error("Should dequeue")
	}
	if valueOne != 1 {
		t.Error(fmt.Sprintf("Wrong value %d", valueOne))
	}
	if valueTwo != 2 {
		t.Error(fmt.Sprintf("Wrong value %d", valueTwo))
	}
	if diff := cmp.Diff([]int{3}, queue.bucket); diff != "" {
		t.Error(diff)
	}
}

func Test_queueInterface(t *testing.T) {
	// Arrange
	queue := newQueueInterface()
	queue.append(1)
	queue.append(2)
	queue.append(3)

	// Act
	var outOne int
	okOne, errOne := queue.tryDequeue(&outOne)
	var outTwo int
	okTwo, errTwo := queue.tryDequeue(&outTwo)

	// Assert
	if !okOne || !okTwo {
		t.Error("Should dequeue")
	}
	if errOne != nil || errTwo != nil {
		t.Error(errOne)
		t.Error(errTwo)
	}
	if outOne != 1 {
		t.Error(fmt.Sprintf("Wrong value %d", outOne))
	}
	if outTwo != 2 {
		t.Error(fmt.Sprintf("Wrong value %d", outTwo))
	}
	if diff := cmp.Diff([]interface{}{3}, queue.bucket); diff != "" {
		t.Error(diff)
	}
}

func Test_queueInterface_errors(t *testing.T) {
	// Arrange
	queue := newQueueInterface()

	// Act
	queue.append(1)
	errAppend := queue.append("foo")
	queue.append(2)
	var out string
	ok, err := queue.tryDequeue(&out)
	okPtr, errPtr := queue.tryDequeue(out)

	// Assert
	if errAppend == nil {
		t.Error("wrong type when append should give error")
	}
	if err == nil || ok {
		t.Error("errors should be given when wrong type as output")
	}
	if errPtr == nil || okPtr {
		t.Error("non pointer output should throw error")
	}
}
