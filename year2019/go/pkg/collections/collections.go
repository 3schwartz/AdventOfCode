package collections

type visited[T comparable] struct {
	visits map[T]struct{}
}

func NewVisited[T comparable]() *visited[T] {
	return &visited[T]{
		visits: map[T]struct{}{},
	}
}

func (v *visited[T]) Contains(other T) bool {
	_, ok := v.visits[other]
	return ok
}

func (v *visited[T]) Add(other T) {
	v.visits[other] = struct{}{}
}
