package io

import (
	"fmt"
	"os"
)

func ReadData(day int) string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/day%d_data.txt", day))
	if err != nil {
		panic(err)
	}
	return string(f)
}
