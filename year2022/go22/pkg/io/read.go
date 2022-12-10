package io

import (
	"fmt"
	"os"
)

func ReadData(file string) string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day%s_data.txt", file))
	if err != nil {
		panic(err)
	}
	return string(f)
}
