package coders

import "math"

type observer interface {
	notify(output int)
}

type FromTo struct {
	From int
	To   int
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

type thrusterSignal func(codes []int, signals signals) int

func findMaxThrusterSignal(codes []int, findThrusterSignal thrusterSignal, fromTo FromTo) int {
	below := fromTo.From - 1
	inputs := signals{below, below, below, below, below}
	maxSignal := math.MinInt

	for first := fromTo.From; first < fromTo.To; first++ {
		inputs.a = first

		for second := fromTo.From; second < fromTo.To; second++ {
			if inputs.contains(second) {
				continue
			}
			inputs.b = second

			for third := fromTo.From; third < fromTo.To; third++ {
				if inputs.contains(third) {
					continue
				}
				inputs.c = third

				for fourth := fromTo.From; fourth < fromTo.To; fourth++ {
					if inputs.contains(fourth) {
						continue
					}
					inputs.d = fourth

					for fifth := fromTo.From; fifth < fromTo.To; fifth++ {
						if inputs.contains(fifth) {
							continue
						}
						inputs.e = fifth
						last := findThrusterSignal(codes, inputs)
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
