package collections

type queue[T any] struct {
	bucket []T
}

func CreateQueue[T any]() *queue[T] {
	return &queue[T]{}
}

func (q *queue[T]) Append(element T) {
	q.bucket = append(q.bucket, element)
}

func (q *queue[T]) TryDequeue() (T, bool) {
	if len(q.bucket) == 0 {
		var dummy T
		return dummy, false
	}
	value := q.bucket[0]
	var zero T
	q.bucket[0] = zero
	q.bucket = q.bucket[1:]
	return value, true
}
