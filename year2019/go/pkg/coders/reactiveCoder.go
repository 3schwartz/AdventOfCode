package coders

type subject interface {
}

type reactiveCoder struct {
	queue queue[int]
}

func ReactiveCoderFindMaxThrusterSignal(codes []int, fromTo FromTo) int {
	return findMaxThrusterSignal(codes, reactiveCoderMaxSignal, fromTo)
}

func reactiveCoderMaxSignal(codes []int, signals signals) int {
	return 0
}

func (rc *reactiveCoder) notify(output int) {

}

type queue[T any] struct {
	bucket []T
}

func newQueue[T any]() queue[T] {
	return queue[T]{
		bucket: []T{},
	}
}

func (q *queue[T]) append(input T) {
	q.bucket = append(q.bucket, input)
}

func (q *queue[T]) tryDequeue() (T, bool) {
	if len(q.bucket) == 0 {
		var dummy T
		return dummy, false
	}
}
