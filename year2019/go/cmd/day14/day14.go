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
	rawCount, _ := chemicals.getRawCountFrom("FUEL", 1, map[string]int{})

	fmt.Printf("Part 1: %d\n", rawCount)
}

func (cs chemicalStore) getRawCountFrom(name string, needed int, stored map[string]int) (rawCount int, produced int) {
	if name == "ORE" {
		return needed, needed
	}
	chemical, ok := cs[name]
	if !ok {
		panic(fmt.Errorf("don't know chemical: %s", name))
	}
	totalRawCount := 0
	for _, input := range chemical.chemicalInputs {
		if stored[input.name] >= input.count {
			stored[input.name] -= input.count
			continue
		}
		askFor := input.count - stored[input.name]
		stored[input.name] = 0
		rawCount, produced := cs.getRawCountFrom(input.name, askFor, stored)
		stored[input.name] += produced - askFor
		totalRawCount += rawCount
	}

	needFactor := int(math.Ceil(float64(needed) / float64(chemical.outputCount)))
	return totalRawCount * needFactor, chemical.outputCount * needFactor
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
