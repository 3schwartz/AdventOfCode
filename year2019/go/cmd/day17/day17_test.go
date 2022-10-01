package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"testing"
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
