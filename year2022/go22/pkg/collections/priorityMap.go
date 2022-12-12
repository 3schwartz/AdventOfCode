package collections

type priorityMap[T comparable] struct {
	priorityMap map[int]map[T]map[T]struct{}
}

func CreatePriorityMap[T comparable]() *priorityMap[T] {
	return &priorityMap[T]{
		priorityMap: make(map[int]map[T]map[T]struct{}, 1),
	}
}

func (pm *priorityMap[T]) Len() int {
	return len(pm.priorityMap)
}

func (pm *priorityMap[T]) CopyVisited(visitedOld map[T]struct{}) map[T]struct{} {
	visitedNew := make(map[T]struct{})
	for v := range visitedOld {
		visitedNew[v] = struct{}{}
	}
	return visitedNew
}

func (pm *priorityMap[T]) Append(item T, priority int, visited map[T]struct{}) {
	itemCollection, ok := pm.priorityMap[priority]
	if !ok {
		itemCollection = make(map[T]map[T]struct{}, 1)
	}
	itemCollection[item] = visited
	pm.priorityMap[priority] = itemCollection
}

func (pm *priorityMap[T]) TryDequeue() (contains bool, priority int, items map[T]map[T]struct{}) {
	if pm.Len() == 0 {
		return false, 0, nil
	}
	var ok bool
	current := 0
	for {
		items, ok = pm.priorityMap[current]
		if !ok {
			current++
			continue
		}
		delete(pm.priorityMap, current)
		break
	}
	return true, current, items
}
