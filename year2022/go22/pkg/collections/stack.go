package collections

type Stack[T any] struct {
	bucket []T
}

func CreateStack[T any]() *Stack[T] {
	return &Stack[T]{}
}

func (q *Stack[T]) Append(element T) {
	q.bucket = append(q.bucket, element)
}

func (q *Stack[T]) TryPop() (T, bool) {
	if len(q.bucket) == 0 {
		var dummy T
		return dummy, false
	}
	value := q.bucket[len(q.bucket)-1]
	var zero T
	q.bucket[len(q.bucket)-1] = zero
	q.bucket = q.bucket[:len(q.bucket)-1]
	return value, true
}

func (q *Stack[T]) Len() int {
	return len(q.bucket)
}
