package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	chemicals := parseChemicals("day14")
	oreCount, _ := chemicals.getRawCountFrom("FUEL", 1, map[string]int{})

	fmt.Printf("Part 1: %d\n", oreCount)

	oreInjected := 1_000_000_000_000
	actualFuelFloor := chemicals.getFuelGivenOre(oreInjected)

	fmt.Printf("Part 2: %d\n", actualFuelFloor)

	likelihoodFuel := chemicals.getFuelGivenOreUsingOptimum(oreInjected)

	fmt.Printf("Part 2 using optimum: %d\n", likelihoodFuel)
}

// deprecated: Just for fun
func (cs chemicalStore) getFuelGivenOreUsingOptimum(oreInjected int) int {
	oreCount, _ := cs.getRawCountFrom("FUEL", 1, map[string]int{})

	lowFuel := oreInjected / oreCount
	highFuel := lowFuel * 10
	for {
		if oreHigh, _ := cs.getRawCountFrom("FUEL", highFuel, map[string]int{}); oreHigh < oreInjected {
			lowFuel = highFuel
			highFuel *= 10
			continue
		}
		break
	}
	var likelihoodFuel int
	for {
		if !(lowFuel < highFuel-1) && highFuel != likelihoodFuel {
			break
		}
		likelihoodFuel = (lowFuel + highFuel) / 2
		oreLikelihood, _ := cs.getRawCountFrom("FUEL", likelihoodFuel, map[string]int{})
		if oreLikelihood > oreInjected {
			highFuel = likelihoodFuel
		}
		if oreLikelihood < oreInjected {
			lowFuel = likelihoodFuel
		}
	}
	return likelihoodFuel
}

func (cs chemicalStore) getFuelGivenOre(oreInjected int) int {
	oreCount, _ := cs.getRawCountFrom("FUEL", 1, map[string]int{})

	expectedFuel := oreInjected / oreCount
	actualOreOnExpectedFuel, _ := cs.getRawCountFrom("FUEL", expectedFuel, map[string]int{})
	oreRatio := float64(oreInjected) / float64(actualOreOnExpectedFuel)
	actualFuelFloor := int(float64(expectedFuel) * oreRatio)
	return actualFuelFloor
}

func (cs chemicalStore) getRawCountFrom(name string, needed int, stored map[string]int) (rawCount int, produced int) {
	if name == "ORE" {
		return needed, needed
	}
	chemical, ok := cs[name]
	if !ok {
		panic(fmt.Errorf("don't know chemical: %s", name))
	}

	needFactor := int(math.Ceil(float64(needed) / float64(chemical.outputCount)))
	totalRawCount := 0
	for _, input := range chemical.chemicalInputs {
		inputNeed := needFactor * input.count
		if stored[input.name] >= inputNeed {
			stored[input.name] -= inputNeed
			continue
		}
		askFor := inputNeed - stored[input.name]
		stored[input.name] = 0
		rawCount, produced := cs.getRawCountFrom(input.name, askFor, stored)
		stored[input.name] += produced - askFor
		totalRawCount += rawCount
	}

	return totalRawCount, chemical.outputCount * needFactor
}

type chemicalStore map[string]reaction

type reaction struct {
	outputCount    int
	chemicalInputs []chemical
}

type chemical struct {
	name  string
	count int
}

func parseChemicals(file string) *chemicalStore {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s_data.txt", file))
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(f), "\r\n")
	chemicals := chemicalStore{}
	for _, line := range lines {
		reactionInput := strings.Split(line, " => ")
		if len(reactionInput) != 2 {
			panic(fmt.Errorf("wrong length of reaction: %s", reactionInput))
		}
		output := strings.Split(reactionInput[1], " ")
		outputCount, err := strconv.Atoi(output[0])
		if err != nil {
			panic(err)
		}
		inputChemicalsStrings := strings.Split(reactionInput[0], ", ")
		inputChemicals := make([]chemical, len(inputChemicalsStrings))
		for i := 0; i < len(inputChemicalsStrings); i++ {
			inputChemical := strings.Split(inputChemicalsStrings[i], " ")
			inputCount, err := strconv.Atoi(inputChemical[0])
			if err != nil {
				panic(err)
			}
			input := chemical{
				name:  inputChemical[1],
				count: inputCount,
			}
			inputChemicals[i] = input
		}
		chemicals[output[1]] = reaction{outputCount, inputChemicals}
	}
	return &chemicals
}
