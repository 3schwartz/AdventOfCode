package main

import (
	"advent/pkg/read"
	"fmt"
	"strconv"
	"strings"
	"testing"
)

func Test_part1(t *testing.T) {
	// Arrange
	data := read.ReadDataAsString("day12")
	moons := createMoonsFromInput(data)
	simulator := createNewMoonSimulator(moons)

	// Act
	simulator.takeSteps(1_000)
	totalEnergy := simulator.getTotalEnergy()

	// Assert
	if totalEnergy != 10845 {
		t.Errorf("Wrong total energy: %d", totalEnergy)
	}
}

func Test_part2Sync(t *testing.T) {
	// Arrange
	data := read.ReadDataAsString("day12")
	moons := createMoonsFromInput(data)
	simulator := createNewMoonSimulator(moons)

	// Act
	stepsToInitial := simulator.stepsToGetBackToInitial()

	// Assert
	if stepsToInitial != 551272644867044 {
		t.Errorf("Wrong steps to get back: %d", stepsToInitial)
	}
}

type Direction int

const (
	X Direction = iota
	Y
	Z
)

type moonSimulator struct {
	moons []*moon
	steps int
}

func createNewMoonSimulator(moons []*moon) *moonSimulator {
	return &moonSimulator{moons: moons}
}

func (ms *moonSimulator) stepsToGetBackToInitial() int64 {
	steps := make([]int, 0)
	directions := [3]Direction{X, Y, Z}
	for _, direction := range directions {
		moonSteps := ms.findStepsToInitialInDirection(direction)
		steps = append(steps, moonSteps)
	}
	yZ := ms.leastCommonMultiple(int64(steps[1]), int64(steps[2]))
	xYZ := ms.leastCommonMultiple(int64(steps[0]), yZ)
	return xYZ
}

func (ms *moonSimulator) leastCommonMultiple(a int64, b int64) int64 {
	return a * b / ms.greatestCommonDivisor(a, b)
}

func (ms *moonSimulator) greatestCommonDivisor(a int64, b int64) int64 {
	for {
		if b == 0 {
			break
		}
		t := a
		a = b
		b = t % b
	}
	return a
}

func (ms *moonSimulator) findStepsToInitialInDirection(direction Direction) int {
	initialPositions := ms.getMoonDirectionState(direction)
	moonSteps := 0
	velocities := map[int][]int{}
	for {
		for i := 0; i < len(ms.moons); i++ {
			velocities[i] = make([]int, 0)
			for j := 0; j < len(ms.moons); j++ {
				if i == j {
					continue
				}
				velocityFromMoonInDirection := ms.moons[i].findVelocityFromMoonInDirection(ms.moons[j], direction)
				velocities[i] = append(velocities[i], velocityFromMoonInDirection)
			}
		}
		for key, pulls := range velocities {
			sum := 0
			for _, p := range pulls {
				sum += p
			}
			ms.moons[key].applyVelocityInDirection(sum, direction)
		}
		moonSteps++
		currentPosition := ms.getMoonDirectionState(direction)
		if ms.isSameState(initialPositions, currentPosition) {
			break
		}
	}

	return moonSteps
}

func (ms *moonSimulator) isSameState(initial []directionPosition, current []directionPosition) bool {
	for i := 0; i < len(initial); i++ {
		if initial[i] != current[i] {
			return false
		}
	}
	return true
}

func (ms *moonSimulator) getMoonDirectionState(direction Direction) []directionPosition {
	positions := make([]directionPosition, 0, len(ms.moons))
	for _, moon := range ms.moons {
		directionPosition := moon.getDirectionPosition(direction)
		positions = append(positions, directionPosition)
	}
	return positions
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

func createMoonsFromInput(data string) []*moon {
	lines := strings.Split(data, "\r\n")
	moons := make([]*moon, 0, len(lines))
	for _, line := range lines {
		moons = append(moons, createNewMoon(line))
	}
	return moons
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

func (m *moon) findVelocityFromMoonInDirection(moonOther *moon, direction Direction) int {
	switch direction {
	case X:
		return m.findPullDirection(m.cX, moonOther.cX)
	case Y:
		return m.findPullDirection(m.cY, moonOther.cY)
	case Z:
		return m.findPullDirection(m.cZ, moonOther.cZ)
	default:
		panic(fmt.Sprintf("direction not mapped: %d", direction))
	}
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

func (m *moon) applyVelocityInDirection(pull int, direction Direction) {
	switch direction {
	case X:
		m.vX += pull
		m.cX += m.vX
	case Y:
		m.vY += pull
		m.cY += m.vY
	case Z:
		m.vZ += pull
		m.cZ += m.vZ
	default:
		panic(fmt.Sprintf("direction not mapped: %d", direction))
	}
}

func (m *moon) getTotalEnergy() int {
	potentialEnergy := abs(m.cX) + abs(m.cY) + abs(m.cZ)
	kineticEnergy := abs(m.vX) + abs(m.vY) + abs(m.vZ)
	return potentialEnergy * kineticEnergy
}

func (m *moon) getDirectionPosition(direction Direction) directionPosition {
	switch direction {
	case X:
		return directionPosition{m.cX, m.vX}
	case Y:
		return directionPosition{m.cY, m.vY}
	case Z:
		return directionPosition{m.cZ, m.vZ}
	default:
		panic(fmt.Sprintf("direction not mapped: %d", direction))
	}
}

type directionPosition struct {
	coordinate, velocity int
}

func abs(value int) int {
	if value < 0 {
		return -1 * value
	}
	return value
}
