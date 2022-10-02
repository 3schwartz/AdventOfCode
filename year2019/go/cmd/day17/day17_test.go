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
	data := []struct {
		name                  string
		route                 string
		expectedMovementLogic movementLogic
	}{
		{
			"singleDigits",
			"R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
			movementLogic{
				[]string{"A", "A", "B", "A", "C", "B", "A", "A", "A", "C"},
				[]string{"R", "8"},
				[]string{"R", "4", "R", "4"},
				[]string{"L", "6", "L", "2"},
			},
		},
		{
			"part2",
			"L,12,L,8,R,12,L,10,L,8,L,12,R,12,L,12,L,8,R,12,R,12,L,8,L,10,L,12,L,8,R,12,L,12,L,8,R,12,R,12,L,8,L,10,L,10,L,8,L,12,R,12,R,12,L,8,L,10,L,10,L,8,L,12,R,12",
			movementLogic{
				[]string{"A", "B", "A", "C", "A", "A", "C", "B", "C", "B"},
				[]string{"L", "12", "L", "8", "R", "12"},
				[]string{"L", "10", "L", "8", "L", "12", "R", "12"},
				[]string{"R", "12", "L", "8", "L", "10"},
			},
		},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			//Arrange
			routeElements := strings.Split(d.route, ",")

			// Act
			movementLogic, err := findMovementLogic(routeElements)

			// Assert
			if err != nil {
				t.Error(err)
			}
			if !reflect.DeepEqual(movementLogic.Routine, d.expectedMovementLogic.Routine) {
				t.Error("routine not equal")
			}
			if !reflect.DeepEqual(movementLogic.A, d.expectedMovementLogic.A) {
				t.Error("A not equal")
			}
			if !reflect.DeepEqual(movementLogic.B, d.expectedMovementLogic.B) {
				t.Error("B not equal")
			}
			if !reflect.DeepEqual(movementLogic.C, d.expectedMovementLogic.C) {
				t.Error("C not equal")
			}
			if diff := cmp.Diff(d.expectedMovementLogic, movementLogic); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_whenGivenRouteFrom_ThenNotError(t *testing.T) {
	//Arrange
	route := "L,12,L,8,R,12,L,10,L,8,L,12,R,12,L,12,L,8,R,12,R,12,L,8,L,10,L,12,L,8,R,12,L,12,L,8,R,12,R,12,L,8,L,10,L,10,L,8,L,12,R,12,R,12,L,8,L,10,L,10,L,8,L,12,R,12"
	routeElements := strings.Split(route, ",")

	// Act
	_, err := findMovementLogic(routeElements)

	// Assert
	if err != nil {
		t.Error(err)
	}
}
