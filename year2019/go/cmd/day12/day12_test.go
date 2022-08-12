package main

import (
	"advent/pkg/read"
	"strconv"
	"strings"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	data := read.ReadDataAsString("day12")
	lines := strings.Split(data, "\r\n")
	moons := make([]*moon, len(lines))
	for _, line := range lines {
		moons = append(moons, createNewMoon(line))
	}
	simulator := createNewMoonSimulator(moons)

	// Act
	simulator.takeSteps(1_000)
	totalEnergy := simulator.getTotalEnergy()

	// Assert
	if totalEnergy != 10845 {
		t.Errorf("Wrong total energy: %d", totalEnergy)
	}
}

type moonSimulator struct {
	moons []*moon
	steps int
}

func createNewMoonSimulator(moons []*moon) *moonSimulator {
	return &moonSimulator{moons: moons}
}

func (ms *moonSimulator) takeSteps(stepCount int) {
	currentSteps := 0
	velocities := map[int][]velocity{}
	for {
		if currentSteps >= stepCount {
			break
		}
		for i, moon := range ms.moons {
			moonVelocities := make([]velocity, 0)
			for _, moonOther := range ms.moons {
				if moon == moonOther {
					continue
				}
				moonVelocity := moon.findVelocityFromMoon(moonOther)
				moonVelocities = append(moonVelocities, moonVelocity)
			}
			velocities[i] = moonVelocities
		}

		currentSteps++
		ms.steps++
	}

}

type velocity struct {
	x, y, z int
}

type moon struct {
	vX, vY, vZ int
	cX, cY, cZ int
}

func createNewMoon(line string) *moon {
	line = strings.TrimPrefix(line, "<")
	line = strings.TrimSuffix(line, ">")
	coords := strings.Split(line, ", ")
	coordPoints := [3]int{}
	for i, coord := range coords {
		coordDef := strings.Split(coord, "=")
		coordPoint, err := strconv.Atoi(string(coordDef[1]))
		if err != nil {
			panic(err)
		}
		coordPoints[i] = coordPoint
	}
	return &moon{
		cX: coordPoints[0],
		cY: coordPoints[1],
		cZ: coordPoints[2],
	}
}
