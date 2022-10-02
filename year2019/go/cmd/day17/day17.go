package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"errors"
	"fmt"
	"reflect"
)

func main() {
	codes := read.ReadData("day17")
	intCoder := coders.ASCIIIntCoder{}
	cameraMap := intCoder.CreateCameraMap(codes)
	scaffoldThreshold := intCoder.FindScaffoldIntersections(cameraMap)

	fmt.Printf("Part 1: %d", scaffoldThreshold)

	intCoder.Print(cameraMap)

	robot, position, err := intCoder.GetRobotPosition(cameraMap)
	if err != nil {
		panic(err)
	}

	movements := intCoder.GetMovements(cameraMap, robot, position)
	printMovements(movements)

	movementLogic, err := findMovementLogic(movements)
	if err != nil {
		panic(err)
	}

	input := createInput(movementLogic)
	codes[0] = "2"
	dust := intCoder.ReportDust(codes, input)

	fmt.Printf("Part 2: %d", dust[len(dust)-1])
}

func printMovements(movements []string) {
	fmt.Println("Print movements:")
	for _, movement := range movements {
		fmt.Print(movement)
	}
	fmt.Println()
	fmt.Println("---")
}

func createInput(logic movementLogic) []int {
	input := make([]int, 0)
	routine := createLine(logic.Routine)
	input = append(input, routine...)
	a := createLine(logic.A)
	input = append(input, a...)
	b := createLine(logic.B)
	input = append(input, b...)
	c := createLine(logic.C)
	input = append(input, c...)
	input = append(input, 'n', 10)
	return input
}

func createLine(line []string) []int {
	input := make([]int, 0)
	lengthLine := len(line)
	for i, routine := range line {
		for _, elm := range routine {
			input = append(input, int(elm))
		}
		if i == lengthLine-1 {
			input = append(input, 10)
			break
		}
		input = append(input, ',')
	}
	return input
}

func findMovementLogic(route []string) (movementLogic, error) {
	length := len(route)
	for a := 1; a <= 10; a++ {
		endA := a
		if endA > length {
			break
		}
		partA := route[0:endA]
		if !isLessThanMemory(partA) {
			break
		}

		aFunc := movementFunction{
			partA,
			len(partA),
		}
		for b := 1; b <= 10; b++ {
			endB := endA + b
			for {
				if endB > length {
					break
				}
				if reflect.DeepEqual(aFunc.input, route[a:endB]) {
					endB += b
					endA += b
					continue
				}
				break
			}
			if endB > length {
				break
			}
			partB := route[endA:endB]
			if !isLessThanMemory(partB) {
				break
			}

			bFunc := movementFunction{
				partB,
				len(partB),
			}
			for c := 1; c <= 10; c++ {
				endC := endB + c
				for {
					if endC > length {
						break
					}
					if reflect.DeepEqual(bFunc.input, route[endB:endC]) || reflect.DeepEqual(aFunc.input, route[endB:endC]) {
						endC += c
						endB += c
						continue
					}
					break
				}
				if endC > length {
					break
				}
				partC := route[endB:endC]
				if !isLessThanMemory(partC) {
					break
				}

				cFunc := movementFunction{
					partC,
					len(partC),
				}

				var foundLength int
				movementLogicResult := movementLogic{}
				for {
					if doesFuncFit(route, aFunc, foundLength, length) {
						movementLogicResult.Routine = append(movementLogicResult.Routine, "A")
						foundLength += aFunc.length
						continue
					}
					if doesFuncFit(route, bFunc, foundLength, length) {
						movementLogicResult.Routine = append(movementLogicResult.Routine, "B")
						foundLength += bFunc.length
						continue
					}
					if doesFuncFit(route, cFunc, foundLength, length) {
						movementLogicResult.Routine = append(movementLogicResult.Routine, "C")
						foundLength += cFunc.length
						continue
					}
					if !isLessThanMemory(movementLogicResult.Routine) {
						break
					}

					if foundLength >= length-1 {
						movementLogicResult.A = aFunc.input
						movementLogicResult.B = bFunc.input
						movementLogicResult.C = cFunc.input
						return movementLogicResult, nil
					}
					break
				}
			}
		}
	}
	return movementLogic{}, errors.New("not able to find movement logic")
}

func isLessThanMemory(sequence []string) bool {
	commas := len(sequence) - 1
	var asciis int
	for _, v := range sequence {
		asciis += len(v)
	}
	return commas+asciis <= 20
}

func doesFuncFit(route []string, moveFunc movementFunction, foundLength int, length int) bool {
	if foundLength+moveFunc.length <= length && reflect.DeepEqual(moveFunc.input, route[foundLength:foundLength+moveFunc.length]) {
		return true
	}
	return false
}

type movementFunction struct {
	input  []string
	length int
}

type movementLogic struct {
	Routine []string
	A       []string
	B       []string
	C       []string
}
