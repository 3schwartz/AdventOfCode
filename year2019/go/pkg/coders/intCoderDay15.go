package coders

import "fmt"

type movement struct {
	input int
	move  Coordinate
}

func (c Coordinate) add(other Coordinate) Coordinate {
	return Coordinate{
		c.x + other.x,
		c.y + other.y,
	}
}

type OxygenFinderIntCoder struct {
	IntCoder
	codes         map[int]int
	position      Coordinate
	movementCount int
}

func CreateOxygenFinderIntCoder(codesInput []int) *OxygenFinderIntCoder {
	codes := make(map[int]int, len(codesInput))
	for i, v := range codesInput {
		codes[i] = v
	}
	return &OxygenFinderIntCoder{
		codes: codes,
	}
}

func (of OxygenFinderIntCoder) getMovements() []movement {
	return []movement{
		{1, Coordinate{0, 1}},
		{2, Coordinate{0, -1}},
		{3, Coordinate{-1, 0}},
		{4, Coordinate{1, 0}},
	}
}

func (of *OxygenFinderIntCoder) GetPosition() Coordinate {
	return of.position
}

func (of *OxygenFinderIntCoder) GetMovementCount() int {
	return of.movementCount
}

func (of *OxygenFinderIntCoder) copy(nextMove Coordinate) OxygenFinderIntCoder {
	codesCopy := make(map[int]int, len(of.codes))
	for key, value := range of.codes {
		codesCopy[key] = value
	}
	return OxygenFinderIntCoder{
		IntCoder: IntCoder{
			idx:          of.idx,
			relativeBase: of.relativeBase,
		},
		codes:         codesCopy,
		position:      of.position.add(nextMove),
		movementCount: of.movementCount,
	}
}

func (of *OxygenFinderIntCoder) updateWithInput(input int, execution int) {
	of.codes[of.getIdxFromMode(of.codes, execution, 1)] = input
	of.idx += 2
	of.movementCount++
}

func (of *OxygenFinderIntCoder) FindOxygen(walls map[Coordinate]bool) (bool, []*OxygenFinderIntCoder) {
optLoop:
	for {
		execution := of.codes[of.idx]
		switch optCode := execution % 100; optCode {
		case 1:
			of.codes[of.getIdxFromMode(of.codes, execution, 3)] =
				of.codes[of.getIdxFromMode(of.codes, execution, 2)] + of.codes[of.getIdxFromMode(of.codes, execution, 1)]
			of.idx += 4
		case 2:
			of.codes[of.getIdxFromMode(of.codes, execution, 3)] = of.codes[of.getIdxFromMode(of.codes, execution, 2)] * of.codes[of.getIdxFromMode(of.codes, execution, 1)]
			of.idx += 4
		case 3:
			newOxygenFinders := make([]*OxygenFinderIntCoder, 0, 4)
			for _, movement := range of.getMovements() {
				nextPlace := of.position.add(movement.move)
				if walls[nextPlace] {
					continue
				}
				oxygenFinderCopy := of.copy(movement.move)
				oxygenFinderCopy.updateWithInput(movement.input, execution)
				newOxygenFinders = append(newOxygenFinders, &oxygenFinderCopy)
			}
			return false, newOxygenFinders
		case 4:
			output := of.codes[of.getIdxFromMode(of.codes, execution, 1)]
			switch output {
			case 0:
				walls[of.position] = true
				return false, nil
			case 1:
				break
			case 2:
				return true, nil
			}
			of.idx += 2
		case 5:
			if of.codes[of.getIdxFromMode(of.codes, execution, 1)] != 0 {
				of.idx = of.codes[of.getIdxFromMode(of.codes, execution, 2)]
				break
			}
			of.idx += 3
		case 6:
			if of.codes[of.getIdxFromMode(of.codes, execution, 1)] == 0 {
				of.idx = of.codes[of.getIdxFromMode(of.codes, execution, 2)]
				break
			}
			of.idx += 3
		case 7:
			var toAssign int
			if of.codes[of.getIdxFromMode(of.codes, execution, 1)] < of.codes[of.getIdxFromMode(of.codes, execution, 2)] {
				toAssign = 1
			}
			of.codes[of.getIdxFromMode(of.codes, execution, 3)] = toAssign
			of.idx += 4
		case 8:
			var toAssign int
			if of.codes[of.getIdxFromMode(of.codes, execution, 1)] == of.codes[of.getIdxFromMode(of.codes, execution, 2)] {
				toAssign = 1
			}
			of.codes[of.getIdxFromMode(of.codes, execution, 3)] = toAssign
			of.idx += 4
		case 9:
			of.relativeBase += of.codes[of.getIdxFromMode(of.codes, execution, 1)]
			of.idx += 2
		case 99:
			break optLoop
		default:
			panic(fmt.Sprintf("OptCode not known: %d", optCode))
		}
	}
	panic("Should not finish with 99")
}
