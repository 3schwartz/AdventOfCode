package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("Hello!")
}

func readData() string {
	f, err := os.ReadFile("../../../data/test.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
