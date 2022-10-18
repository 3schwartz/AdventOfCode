package collections

type priorityMap[T comparable] struct {
	priorityMap   map[int]map[T]struct{}
	priorityState int
}

func CreatePriorityMap[T comparable]() *priorityMap[T] {
	return &priorityMap[T]{
		priorityMap: make(map[int]map[T]struct{}, 1),
	}
}

func (pm *priorityMap[T]) Len() int {
	return len(pm.priorityMap)
}

func (pm *priorityMap[T]) Append(item T, priority int) {
	itemCollection, ok := pm.priorityMap[priority]
	if !ok {
		itemCollection = make(map[T]struct{}, 1)
	}
	itemCollection[item] = struct{}{}
	pm.priorityMap[priority] = itemCollection
}

func (pm *priorityMap[T]) TryDequeue() (contains bool, priority int, items map[T]struct{}) {
	if pm.Len() == 0 {
		return false, 0, nil
	}
	var ok bool
	var current int
	for {
		current = pm.priorityState
		pm.priorityState++

		items, ok = pm.priorityMap[current]
		if !ok {
			continue
		}
		delete(pm.priorityMap, current)
		break
	}
	return true, current, items
}
