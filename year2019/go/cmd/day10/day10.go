package main

import (
	"advent/pkg/collections"
	"advent/pkg/read"
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"strings"
)

func main() {
	data := read.ReadDataAsString("day10")
	asteroidMap := newAsteroidMap(data)
	monitoringStation := newMonitoringStation(asteroidMap)

	actual := monitoringStation.findLocationWithMaxDetectedAsteroids()
	formatted, _ := json.Marshal(&actual)

	fmt.Printf("Part 1: Coordinate %s\n", string(formatted))

	vaporized := monitoringStation.vaporizeAsteroids(actual.Coordinate)

	fmt.Printf("Part 2: %d", vaporized[199].X*100+vaporized[199].Y)
}

type Coordinate struct {
	X int
	Y int
}

func (c *Coordinate) angle(other Coordinate) float64 {
	x, y := c.changeOriginInCoordinateSystem(other)
	x, y = c.mirrorAroundSouthwestNortheast(x, y)

	// Atan2 has values within (-pi, pi). Pi when x<0 and y->0 and -Pi when x<0 and -y->0.
	// Hence Atan2 is decreasing from Pi when x<0 and y->0 clockwise to -Pi when x<0 and -y->0.
	// The above mirror in coordinate system have placed the origin 12 o'clock
	// (which originally was downwards along y axis in exercise and downwards along -y after shift)
	// along -x axis.
	// We want to order angles clockwise from -x where angles close to -x where y->0 gets lowest values
	// and when -y->0 gets highest values.
	// atan2 calculated angle between point and x axis hence angles close to Pi when x<0 and y->0.
	// When we negate atan2 we instead get a increasing function clockwise from x<0 which is
	// exactly what we need.
	angle := -math.Atan2(y, x)
	return angle
}

// Change origin in coordinate system
func (c *Coordinate) changeOriginInCoordinateSystem(other Coordinate) (float64, float64) {
	return float64(other.X - c.X), float64(other.Y - c.Y)
}

// mirror x & y around the southwest / northeast axis to shift from a left-handed to right-handed
// coordinate system
func (c *Coordinate) mirrorAroundSouthwestNortheast(x float64, y float64) (float64, float64) {
	return y, x
}

func (c *Coordinate) manhattanDistance(other Coordinate) float64 {
	distance := math.Abs(float64(c.X-other.X)) + math.Abs(float64(c.Y-other.Y))
	return distance
}

type MonitoringLocation struct {
	Coordinate
	DetectedAsteroids int
}

type slope struct {
	x int
	y int
}

type vaporize struct {
	coordinate Coordinate
	angle      float64
}

type monitoringStation struct {
	asteroids *asteroidMap
}

func newMonitoringStation(asteroids *asteroidMap) *monitoringStation {
	return &monitoringStation{asteroids: asteroids}
}

func (ms *monitoringStation) vaporizeAsteroids(coord Coordinate) []Coordinate {
	asteroids := ms.asteroids.copy()
	vaporized := make([]Coordinate, 0)

	for len(asteroids.asteroids) > 1 {
		vaporizedInCycle := ms.vaporizeCycle(coord, asteroids)
		for _, v := range vaporizedInCycle {
			vaporized = append(vaporized, v.coordinate)
		}
	}

	return vaporized
}

// Mutate asteroids map
func (ms *monitoringStation) vaporizeCycle(center Coordinate, asteroids *asteroidMap) []vaporize {
	vaporized := make([]vaporize, 0)

	angles := ms.findNearestAsteroids(center, asteroids)

	for o, value := range angles {
		vaporized = append(vaporized, vaporize{coordinate: value, angle: o})
	}

	sort.Slice(vaporized, func(i, j int) bool {
		return vaporized[i].angle < vaporized[j].angle
	})

	for _, v := range vaporized {
		delete(asteroids.asteroids, v.coordinate)
	}

	return vaporized
}

func (ms *monitoringStation) findNearestAsteroids(center Coordinate, asteroids *asteroidMap) map[float64]Coordinate {
	angles := map[float64]Coordinate{}

	for coord := range asteroids.asteroids {
		if coord.X == center.X && coord.Y == center.Y {
			continue
		}
		angle := center.angle(coord)
		previous, ok := angles[angle]
		if !ok {
			angles[angle] = coord
			continue
		}
		if center.manhattanDistance(coord) < center.manhattanDistance(previous) {
			angles[angle] = coord
		}
	}

	return angles
}

func (ms *monitoringStation) getSlopesFromCenterInOrder(coord Coordinate) []slope {
	slopes := make([]slope, 0)
	// Lower right
	for i := 0; i <= ms.asteroids.cols-coord.X; i++ {
		for j := 0; j <= ms.asteroids.rows-coord.Y; j++ {
			slopes = append(slopes, slope{i, j})
		}
	}
	// Upper left
	for i := 0; -coord.X <= i; i-- {
		for j := 0; -coord.Y <= j; j-- {
			slopes = append(slopes, slope{i, j})
		}
	}
	// Upper right
	for i := 0; i <= ms.asteroids.cols-coord.X; i++ {
		for j := 0; -coord.Y <= j; j-- {
			slopes = append(slopes, slope{i, j})
		}
	}
	// Lower left
	for i := 0; -coord.X <= i; i-- {
		for j := 0; j <= ms.asteroids.rows-coord.Y; j++ {
			slopes = append(slopes, slope{i, j})
		}
	}
	return slopes
}

func (ms *monitoringStation) findLocationWithMaxDetectedAsteroidsNew() MonitoringLocation {
	location := MonitoringLocation{
		DetectedAsteroids: math.MinInt,
	}
	for key := range ms.asteroids.asteroids {

		nearest := ms.findNearestAsteroids(key, ms.asteroids)

		if len(nearest) > location.DetectedAsteroids {
			location.Coordinate = key
			location.DetectedAsteroids = len(nearest)
		}
	}

	return location
}

func (ms *monitoringStation) findLocationWithMaxDetectedAsteroids() MonitoringLocation {
	location := MonitoringLocation{
		DetectedAsteroids: math.MinInt,
	}
	for key := range ms.asteroids.asteroids {
		slopes := ms.getSlopesFromCenterInOrder(key)
		var detectedAsteroids int
		visitedCoordinates := collections.NewVisited[Coordinate]()
		for _, slope := range slopes {
			if slope.x == 0 && slope.y == 0 {
				continue
			}
			lookupX := key.X
			lookupY := key.Y
			var foundOnPath bool

			for {
				lookupX += slope.x
				lookupY += slope.y
				if lookupX < 0 || lookupX > ms.asteroids.cols {
					break
				}
				if lookupY < 0 || lookupY > ms.asteroids.rows {
					break
				}

				coord := Coordinate{lookupX, lookupY}
				if visitedCoordinates.Contains(coord) {
					break
				}
				visitedCoordinates.Add(coord)
				if foundOnPath {
					continue
				}
				if ms.asteroids.contains(coord) {
					foundOnPath = true
				}
			}
			if foundOnPath {
				detectedAsteroids += 1
			}
		}
		if detectedAsteroids > location.DetectedAsteroids {
			location.Coordinate = key
			location.DetectedAsteroids = detectedAsteroids
		}
	}

	return location
}

type asteroidMap struct {
	asteroids map[Coordinate]bool
	// Deprecated
	cols int
	// Deprecated
	rows int
}

func newAsteroidMap(data string) *asteroidMap {
	lines := strings.Split(data, "\r\n")
	asteroidsMap := map[Coordinate]bool{}
	for j, line := range lines {
		for i, r := range line {
			if r == '#' {
				asteroidsMap[Coordinate{i, j}] = true
			}
		}
	}
	asteroids := asteroidMap{
		cols:      len(lines),
		rows:      len(lines[0]),
		asteroids: asteroidsMap,
	}
	return &asteroids
}

func (am *asteroidMap) contains(coord Coordinate) bool {
	_, ok := am.asteroids[coord]
	return ok
}

func (am *asteroidMap) copy() *asteroidMap {
	dst := make(map[Coordinate]bool, len(am.asteroids))
	for key, value := range am.asteroids {
		dst[key] = value
	}
	return &asteroidMap{
		cols:      am.cols,
		rows:      am.rows,
		asteroids: dst,
	}
}
