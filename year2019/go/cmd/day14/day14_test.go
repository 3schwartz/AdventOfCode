package main

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"testing"
)

func Test_correct_read(t *testing.T) {
	// Arrange & Act
	chemicals := parseChemicals("day14_test")

	// Assert
	if len(*chemicals) != 6 {
		s, _ := json.MarshalIndent(chemicals, "", "\t")
		t.Errorf("chemicals not constructed correct: %s", string(s))
	}
}

func Test_correct_raw_count(t *testing.T) {
	// Arrange
	chemicals := parseChemicals("day14_test")

	// Act
	rawCount := chemicals.getRawCountFrom("FUEL", 1)

	// Assert
	if rawCount != 31 {
		t.Error(rawCount)
	}
}

func (cs chemicalStore) getRawCountFrom(name string, amount int) int {
	if name == "ORE" {
		return amount
	}
	chemical, ok := cs[name]
	if !ok {
		panic(fmt.Errorf("don't know chemical: %s", name))
	}
	totalRawCount := 0
	for _, input := range chemical.chemicalInputs {
		rawCount := cs.getRawCountFrom(input.name, input.count)
		totalRawCount += rawCount
	}
	needAmount := int(math.Ceil(float64(amount) / float64(chemical.outputCount)))
	return totalRawCount * needAmount
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
