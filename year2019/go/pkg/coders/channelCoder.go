package coders

import (
	"context"
	"fmt"
	"math"
)

type channelCoder struct {
	reader     chan int
	codes      []int
	idx        int
	identifier string
	output     []int
	observer   observer
}

func newChannelCoder(identifier string, codesInput []int, inputs []int) channelCoder {
	codes := make([]int, len(codesInput))
	copy(codes, codesInput)
	reader := make(chan int, 10+len(inputs))
	for _, v := range inputs {
		reader <- v
	}
	return channelCoder{
		reader:     reader,
		codes:      codes,
		idx:        0,
		identifier: identifier,
	}
}

// notify implements observer
func (cc *channelCoder) notify(output int) {
	go func() {
		cc.reader <- output
	}()
}

func (cc *channelCoder) getLatestSignal() int {
	return cc.output[len(cc.output)-1]
}

func (cc *channelCoder) attach(ctx context.Context, obs observer) <-chan struct{} {
	cc.observer = obs
	done := make(chan struct{})
	go cc.run(ctx, done)
	return done
}

func (cc *channelCoder) run(ctx context.Context, done chan<- struct{}) {
	for {
		select {
		case input, ok := <-cc.reader:
			if !ok {
				cc.reader = nil // turn this case when channel closed
				continue
			}
			inputUsed := false
		optLoop:
			for {
				execution := cc.codes[cc.idx]
				switch optCode := execution % 100; optCode {
				case 1:
					cc.codes[cc.getIdxFromMode(execution, 3, cc.idx)] =
						cc.codes[cc.getIdxFromMode(execution, 2, cc.idx)] + cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)]
					cc.idx += 4
				case 2:
					cc.codes[cc.getIdxFromMode(execution, 3, cc.idx)] =
						cc.codes[cc.getIdxFromMode(execution, 2, cc.idx)] * cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)]
					cc.idx += 4
				case 3:
					if inputUsed {
						break optLoop
					}
					cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)] = input
					cc.idx += 2
					inputUsed = true
				case 4:
					output := cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)]
					cc.output = append(cc.output, output)
					cc.idx += 2
					cc.observer.notify(output)
				case 5:
					if cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)] != 0 {
						cc.idx = cc.codes[cc.getIdxFromMode(execution, 2, cc.idx)]
						break
					}
					cc.idx += 3
				case 6:
					if cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)] == 0 {
						cc.idx = cc.codes[cc.getIdxFromMode(execution, 2, cc.idx)]
						break
					}
					cc.idx += 3
				case 7:
					var toAssign int
					if cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)] < cc.codes[cc.getIdxFromMode(execution, 2, cc.idx)] {
						toAssign = 1
					}
					cc.codes[cc.getIdxFromMode(execution, 3, cc.idx)] = toAssign
					cc.idx += 4
				case 8:
					var toAssign int
					if cc.codes[cc.getIdxFromMode(execution, 1, cc.idx)] == cc.codes[cc.getIdxFromMode(execution, 2, cc.idx)] {
						toAssign = 1
					}
					cc.codes[cc.getIdxFromMode(execution, 3, cc.idx)] = toAssign
					cc.idx += 4
				case 99:
					done <- struct{}{}
					return
				default:
					panic(fmt.Sprintf("OptCode not known: %d", optCode))
				}
			}
		case <-ctx.Done():
			return
		}
	}
}

func (cc *channelCoder) getIdxFromMode(execution int, parameterPosition int, idx int) int {
	mode := execution / int(math.Pow(10, 1+float64(parameterPosition)))
	mode %= 10
	if mode == 1 {
		return idx + parameterPosition
	}
	return cc.codes[idx+parameterPosition]
}

func ChannelCoderFindMaxThrusterSignal(codes []int, fromTo FromTo) int {
	return findMaxThrusterSignal(codes, channelCoderMaxSignal, fromTo)
}

func channelCoderMaxSignal(codes []int, signals signals) int {
	a := newChannelCoder("a", codes, []int{signals.a, 0})
	b := newChannelCoder("b", codes, []int{signals.b})
	c := newChannelCoder("c", codes, []int{signals.c})
	d := newChannelCoder("d", codes, []int{signals.d})
	e := newChannelCoder("e", codes, []int{signals.e})
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	_ = a.attach(ctx, &b)
	_ = b.attach(ctx, &c)
	_ = c.attach(ctx, &d)
	_ = d.attach(ctx, &e)
	doneE := e.attach(ctx, &a)

	<-doneE

	return e.getLatestSignal()
}
