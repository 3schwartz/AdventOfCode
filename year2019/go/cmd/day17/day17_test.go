package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"errors"
	"reflect"
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_Part1(t *testing.T) {
	// Arrange
	codes := read.ReadData("day17")
	intCoder := coders.ASCIIIntCoder{}
	cameraMap := intCoder.CreateCameraMap(codes)

	// Act
	scaffoldThreshold := intCoder.FindScaffoldIntersections(cameraMap)

	// Assert
	if scaffoldThreshold != 10632 {
		t.Errorf("wrong output: %d", scaffoldThreshold)
	}
}

func Test_givenRoute_WhenFindingRoutineAndFunctions_ThenCorrect(t *testing.T) {
	// Arrange
	expectedMovementLogic := movementLogic{
		[]string{"A", "B", "C", "B", "A", "C"},
		[]string{"R", "8", "R", "8"},
		[]string{"R", "4", "R", "4"},
		[]string{"R", "8", "L", "6", "L", "2"},
	}

	route := "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
	routeElements := strings.Split(route, ",")

	// Act
	movementLogic, err := findMovementLogic(routeElements)

	// Assert
	if err != nil {
		t.Error(err)
	}
	if !reflect.DeepEqual(movementLogic.Routine, expectedMovementLogic.Routine) {
		t.Error("routine not equal")
	}
	if !reflect.DeepEqual(movementLogic.A, expectedMovementLogic.A) {
		t.Error("A not equal")
	}
	if !reflect.DeepEqual(movementLogic.B, expectedMovementLogic.B) {
		t.Error("B not equal")
	}
	if !reflect.DeepEqual(movementLogic.C, expectedMovementLogic.C) {
		t.Error("C not equal")
	}
	if diff := cmp.Diff(expectedMovementLogic, movementLogic); diff != "" {
		t.Error(diff)
	}
}

func findMovementLogic(route []string) (movementLogic, error) {
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
