package main

import (
	"encoding/json"
	"fmt"
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
	rawCount := chemicals.getRawCountFrom("FUEL")

	// Assert
	if rawCount != 31 {
		t.Error(rawCount)
	}
}

func (cs *chemicalStore) getRawCountFrom(s string) int {
	chemical, ok := cs[s]

}

type chemicalStore map[chemical][]chemical

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
		reaction := strings.Split(line, " => ")
		if len(reaction) != 2 {
			panic(fmt.Errorf("wrong length of reaction: %s", reaction))
		}
		output := strings.Split(reaction[1], " ")
		outputCount, err := strconv.Atoi(output[0])
		if err != nil {
			panic(err)
		}
		outputChemical := chemical{
			name:  output[1],
			count: outputCount,
		}
		inputChemicalsStrings := strings.Split(reaction[0], ", ")
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
		chemicals[outputChemical] = inputChemicals
	}
	return &chemicals
}
