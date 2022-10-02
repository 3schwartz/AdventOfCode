package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
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
