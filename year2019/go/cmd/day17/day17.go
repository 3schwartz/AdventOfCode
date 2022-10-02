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

	// intCoder.Print(cameraMap)

	robot, position, err := intCoder.GetRobotPosition(cameraMap)
	if err != nil {
		panic(err)
	}

	movements := intCoder.GetMovements(cameraMap, robot, position)

	fmt.Println("Print movements:")
	for _, movement := range movements {
		fmt.Print(movement)

	}
	fmt.Println()
	fmt.Println("---")

	movementLogic, err := findMovementLogic(movements)
	if err != nil {
		panic(err)
	}
	input := createInput(movementLogic)
	codes[0] = "2"
	dust, err := intCoder.ReportDust(codes, input)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 2: %d", dust)

	fmt.Println(len("10"))

	testSlice := []int{1, 2, 3}
	fmt.Println(len(testSlice))
	fmt.Println(testSlice[1:3])
	// TODO: Find patterns
	bar := "R"
	foo := 'R'
	fmt.Println(bar)
	fmt.Println(bar[0])
	fmt.Println(foo)

}

func createInput(logic movementLogic) []int {
	input := make([]int, 0)
	input = append(input, createLine(logic.Routine)...)
	input = append(input, createLine(logic.A)...)
	input = append(input, createLine(logic.B)...)
	input = append(input, createLine(logic.C)...)
	return input
}

func createLine(line []rune) []int {
	input := make([]int, 0)
	lengthLine := len(line)
	for i, routine := range line {
		input = append(input, int(routine))
		if i == lengthLine {
			input = append(input, 10)
			break
		}
		input = append(input, 44)
	}
	return input
}

func findMovementLogic(route []rune) (movementLogic, error) {
	length := len(route)
	for a := 1; a <= 10; a++ {
		if a > length {
			break
		}
		aFunc := movementFunction{
			route[0:a],
			len(route[0:a]),
		}
		for b := 1; b <= 10; b++ {
			endB := a + b
			if endB > length {
				break
			}
			bFunc := movementFunction{
				route[a:endB],
				len(route[a:endB]),
			}
			for c := 1; c <= 10; c++ {
				endC := a + b + c
				if endC > length {
					break
				}
				cFunc := movementFunction{
					route[endB:endC],
					len(route[endB:endC]),
				}

				var foundLength int
				movementLogicResult := movementLogic{}
				for {
					if doesFuncFit(route, aFunc, foundLength, length) {
						movementLogicResult.Routine = append(movementLogicResult.Routine, 'A')
						foundLength += aFunc.length
						continue
					}
					if doesFuncFit(route, bFunc, foundLength, length) {
						movementLogicResult.Routine = append(movementLogicResult.Routine, 'B')
						foundLength += bFunc.length
						continue
					}
					if doesFuncFit(route, cFunc, foundLength, length) {
						movementLogicResult.Routine = append(movementLogicResult.Routine, 'C')
						foundLength += cFunc.length
						continue
					}
					if foundLength >= length {
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

func doesFuncFit(route []rune, moveFunc movementFunction, foundLength int, length int) bool {
	if foundLength+moveFunc.length <= length && reflect.DeepEqual(moveFunc.input, route[foundLength:foundLength+moveFunc.length]) {
		return true
	}
	return false
}

type movementFunction struct {
	input  []rune
	length int
}

type movementLogic struct {
	Routine []rune
	A       []rune
	B       []rune
	C       []rune
}
