package collections

type Queue[T any] struct {
	bucket []T
}

func CreateQueue[T any]() *Queue[T] {
	return &Queue[T]{}
}

func (q *Queue[T]) Append(element T) {
	q.bucket = append(q.bucket, element)
}

func (q *Queue[T]) TryDequeue() (T, bool) {
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

func (q *Queue[T]) Len() int {
	return len(q.bucket)
}
