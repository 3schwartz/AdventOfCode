package read

import (
	"fmt"
	"os"
	"strings"
)

func ReadData(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s_data.txt", fileName))
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), ",")
}

func ReadDataAsString(fileName string) string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s_data.txt", fileName))
	if err != nil {
		panic(err)
	}
	return string(f)
}
