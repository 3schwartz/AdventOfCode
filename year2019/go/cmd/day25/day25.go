package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"bufio"
	"context"
	"fmt"
	"os"
	"strings"
)

func main() {
	codes := read.ReadData("day25")
	intCodes := coders.ParseIntCodes(codes)
	coder, inputChannel, outputChannel := coders.CreateDroidCoder()
	inputCodes := coder.GenerateCodes(intCodes)

	cts := context.Background()
	ctsWithCancellation, cancel := context.WithCancel(cts)
	defer cancel()

	go coder.FindSanta(inputCodes, ctsWithCancellation)

	for {
		for {
			output := make([]rune, 0)
			for out := range outputChannel {
				output = append(output, rune(out))
				if out == 10 {
					break
				}
			}
			outputString := string(output)
			fmt.Print(outputString)
			if outputString == "Command?\n" {
				break
			}
		}

		reader := bufio.NewReader(os.Stdin)
		fmt.Print("Enter text: ")
		text, _ := reader.ReadString('\n')
		text = strings.Replace(text, "\n", "", -1)
		fmt.Printf("You entered: %s\n", text)
		for _, v := range text {
			if v == 13 || v == 10 {
				continue
			}
			inputChannel <- int(v)
		}
		inputChannel <- 10
	}
}
