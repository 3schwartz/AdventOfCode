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
	moons := make([]*moon, 0, len(lines))
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

		ms.findVelocitiesInStep(velocities)

		ms.applyVelocityInStep(velocities)

		currentSteps++
		ms.steps++
	}
}

func (ms *moonSimulator) findVelocitiesInStep(velocities map[int][]velocity) {
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
}

func (ms *moonSimulator) applyVelocityInStep(velocities map[int][]velocity) {
	for i, moonVelocities := range velocities {
		pull := velocity{}
		for _, moonVelocity := range moonVelocities {
			pull = velocity{pull.x + moonVelocity.x, pull.y + moonVelocity.y, pull.z + moonVelocity.z}
		}
		ms.moons[i].applyVelocity(pull)
	}
}

func (ms *moonSimulator) getTotalEnergy() int {
	var totalEnergy int
	for _, moon := range ms.moons {
		totalEnergy += moon.getTotalEnergy()
	}
	return totalEnergy
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

func (m *moon) findVelocityFromMoon(moonOther *moon) velocity {
	x := m.findPullDirection(m.cX, moonOther.cX)
	y := m.findPullDirection(m.cY, moonOther.cY)
	z := m.findPullDirection(m.cZ, moonOther.cZ)
	return velocity{x, y, z}
}

func (m *moon) findPullDirection(current int, other int) int {
	if current > other {
		return -1
	}
	if current < other {
		return 1
	}
	return 0
}

func (m *moon) applyVelocity(moonVelocity velocity) {
	m.vX += moonVelocity.x
	m.vY += moonVelocity.y
	m.vZ += moonVelocity.z

	m.cX += m.vX
	m.cY += m.vY
	m.cZ += m.vZ
}

func (m *moon) getTotalEnergy() int {
	potentialEnergy := abs(m.cX) + abs(m.cY) + abs(m.cZ)
	kineticEnergy := abs(m.vX) + abs(m.vY) + abs(m.vZ)
	return potentialEnergy * kineticEnergy
}

func abs(value int) int {
	if value < 0 {
		return -1 * value
	}
	return value
}
