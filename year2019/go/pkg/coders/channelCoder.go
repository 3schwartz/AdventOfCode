package coders

import "math"

type channelCoder struct {
	reader chan int
}

func (cc *channelCoder) Run() {
	for v := range cc.reader {

	}
}

func ChannelCoderFindMaxThrusterSignal(codes []int) int {
	return 0
}

type signals struct {
	a int
	b int
	c int
	d int
	e int
}

func (s signals) contains(value int) bool {
	if s.a == value || s.b == value || s.c == value || s.d == value || s.e == value {
		return true
	}
	return false
}

type fromTo struct {
	from int
	to   int
}

type thrusterSignal func(codes []int, signals signals, signalCh chan<- int)

func findMaxThrusterSignal(codes []int, findThrusterSignal thrusterSignal, fromTo fromTo) int {
	below := fromTo.from - 1
	inputs := signals{below, below, below, below, below}
	maxSignal := math.MinInt
	thrusterSignalCh := make(chan int)

	for first := fromTo.from; first < fromTo.to; first++ {
		inputs.a = first

		for second := fromTo.from; second < fromTo.to; second++ {
			if inputs.contains(second) {
				continue
			}
			inputs.b = second

			for third := fromTo.from; third < fromTo.to; third++ {
				if inputs.contains(third) {
					continue
				}
				inputs.c = third

				for fourth := fromTo.from; fourth < fromTo.to; fourth++ {
					if inputs.contains(fourth) {
						continue
					}
					inputs.d = fourth

					for fifth := fromTo.from; fifth < fromTo.to; fifth++ {
						if inputs.contains(fifth) {
							continue
						}
						inputs.e = fifth
						go findThrusterSignal(codes, inputs, thrusterSignalCh)
						last := <-thrusterSignalCh
						if last > maxSignal {
							maxSignal = last
						}
						inputs.e = below
					}
					inputs.d = below
				}
				inputs.c = below
			}
			inputs.b = below
		}
		inputs.a = below
	}
	return maxSignal
}
