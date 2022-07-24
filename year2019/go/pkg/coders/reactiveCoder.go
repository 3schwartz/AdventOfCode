package coders

import (
	"errors"
	"fmt"
	"reflect"
)

type subject interface {
	attach(observer observer)
}

type reactiveCoder struct {
	queue      *queue[int]
	codes      []int
	idx        int
	identifier string
	output     []int
	observer
}

func newReactiveCoder(identifier string, codesInput []int, inputs []int) reactiveCoder {
	codes := make([]int, len(codesInput))
	copy(codes, codesInput)
	queue := newQueue[int]()
	for _, v := range inputs {
		queue.append(v)
	}
	return reactiveCoder{
		queue:      queue,
		codes:      codes,
		idx:        0,
		identifier: identifier,
	}
}

func (rc *reactiveCoder) getLatestSignal() int {
	return rc.output[len(rc.output)-1]
}

// attach implements subject
func (rc *reactiveCoder) attach(observer observer) {
	rc.observer = observer
}

// notify implements observer
func (rc *reactiveCoder) notify(output int) {
	rc.queue.append(output)
optLoop:
	for {
		execution := rc.codes[rc.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			rc.codes[rc.getIdxFromMode(execution, 3, rc.idx)] =
				rc.codes[rc.getIdxFromMode(execution, 2, rc.idx)] + rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)]
			rc.idx += 4
		case 2:
			rc.codes[rc.getIdxFromMode(execution, 3, rc.idx)] =
				rc.codes[rc.getIdxFromMode(execution, 2, rc.idx)] * rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)]
			rc.idx += 4
		case 3:
			input, ok := rc.queue.tryDequeue()
			if !ok {
				break optLoop
			}
			rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)] = input
			rc.idx += 2
		case 4:
			output := rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)]
			rc.output = append(rc.output, output)
			rc.idx += 2
			rc.observer.notify(output)
		case 5:
			if rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)] != 0 {
				rc.idx = rc.codes[rc.getIdxFromMode(execution, 2, rc.idx)]
				break
			}
			rc.idx += 3
		case 6:
			if rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)] == 0 {
				rc.idx = rc.codes[rc.getIdxFromMode(execution, 2, rc.idx)]
				break
			}
			rc.idx += 3
		case 7:
			var toAssign int
			if rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)] < rc.codes[rc.getIdxFromMode(execution, 2, rc.idx)] {
				toAssign = 1
			}
			rc.codes[rc.getIdxFromMode(execution, 3, rc.idx)] = toAssign
			rc.idx += 4
		case 8:
			var toAssign int
			if rc.codes[rc.getIdxFromMode(execution, 1, rc.idx)] == rc.codes[rc.getIdxFromMode(execution, 2, rc.idx)] {
				toAssign = 1
			}
			rc.codes[rc.getIdxFromMode(execution, 3, rc.idx)] = toAssign
			rc.idx += 4
		case 99:
			return
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}
}

func (rc *reactiveCoder) getIdxFromMode(execution int, parameterPosition int, idx int) int {
	mode := execution / intPow(10, 1+parameterPosition) // int(math.Pow(10, 1+float64(parameterPosition)))
	mode %= 10
	if mode == 1 {
		return idx + parameterPosition
	}
	return rc.codes[idx+parameterPosition]
}

func intPow(n, m int) int {
	if m == 0 {
		return 1
	}
	result := n
	for i := 2; i <= m; i++ {
		result *= n
	}
	return result
}

func ReactiveCoderFindMaxThrusterSignal(codes []int, fromTo FromTo) int {
	return findMaxThrusterSignal(codes, reactiveCoderMaxSignal, fromTo)
}

func reactiveCoderMaxSignal(codes []int, signals signals) int {
	a := newReactiveCoder("a", codes, []int{signals.a})
	b := newReactiveCoder("b", codes, []int{signals.b})
	c := newReactiveCoder("c", codes, []int{signals.c})
	d := newReactiveCoder("d", codes, []int{signals.d})
	e := newReactiveCoder("e", codes, []int{signals.e})

	a.attach(&b)
	b.attach(&c)
	c.attach(&d)
	d.attach(&e)
	e.attach(&a)

	a.notify(0)

	return e.getLatestSignal()
}

type queue[T any] struct {
	bucket []T
}

func newQueue[T any]() *queue[T] {
	return &queue[T]{
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
	value := q.bucket[0]
	q.bucket = q.bucket[1:]
	return value, true
}

type queueInterface struct {
	bucket []interface{}
}

func newQueueInterface() *queueInterface {
	return &queueInterface{
		bucket: []interface{}{},
	}
}

func (q *queueInterface) append(input interface{}) error {
	if len(q.bucket) != 0 && reflect.TypeOf(q.bucket[0]) != reflect.TypeOf(input) {
		return errors.New("input type not same as those already in queue")
	}
	q.bucket = append(q.bucket, input)
	return nil
}

func (q *queueInterface) tryDequeue(out interface{}) (bool, error) {
	if len(q.bucket) == 0 {
		return false, nil
	}

	valuePtr := reflect.ValueOf(out)
	if valuePtr.Kind() != reflect.Ptr {
		return false, errors.New("must be a pointer")
	}

	value := q.bucket[0]
	if valuePtr.Elem().Type() != reflect.TypeOf(value) {
		return false, errors.New("output must be of same type as queue elements")
	}

	q.bucket = q.bucket[1:]
	valuePtr.Elem().Set(reflect.ValueOf(value))
	return true, nil
}
