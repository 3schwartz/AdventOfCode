package main

import (
	"advent/pkg/read"
	"testing"

	"github.com/google/go-cmp/cmp"
)

var blackholeTrigonometric, blackholeSlopes MonitoringLocation

func Benchmark_detectAsteroids(b *testing.B) {
	data := read.ReadDataAsString("day10")
	asteroidMap := newAsteroidMap(data)
	monitoringStation := newMonitoringStation(asteroidMap)

	b.Run("Trigonometric", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackholeTrigonometric = monitoringStation.findLocationWithMaxDetectedAsteroidsNew()
		}
	})
	b.Run("Slopes", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackholeSlopes = monitoringStation.findLocationWithMaxDetectedAsteroids()
		}
	})

}

func Test_givenMapThenCorrectDetectedAsteroidsUsingTrigonometric(t *testing.T) {
	data := []struct {
		fileName string
		expected MonitoringLocation
	}{
		{
			"day10",
			MonitoringLocation{Coordinate{19, 14}, 274},
		},
		{
			"day10_test",
			MonitoringLocation{Coordinate{3, 4}, 8},
		},
		{
			"day10_test2",
			MonitoringLocation{Coordinate{11, 13}, 210},
		},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			data := read.ReadDataAsString(d.fileName)
			asteroidMap := newAsteroidMap(data)
			monitoringStation := newMonitoringStation(asteroidMap)

			// Act
			actual := monitoringStation.findLocationWithMaxDetectedAsteroidsNew()

			// Assert
			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_givenMapThenCorrectDetectedAsteroids(t *testing.T) {
	data := []struct {
		fileName string
		expected MonitoringLocation
	}{
		{
			"day10",
			MonitoringLocation{Coordinate{19, 14}, 274},
		},
		{
			"day10_test",
			MonitoringLocation{Coordinate{3, 4}, 8},
		},
		{
			"day10_test2",
			MonitoringLocation{Coordinate{11, 13}, 210},
		},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			data := read.ReadDataAsString(d.fileName)
			asteroidMap := newAsteroidMap(data)
			monitoringStation := newMonitoringStation(asteroidMap)

			// Act
			actual := monitoringStation.findLocationWithMaxDetectedAsteroids()

			// Assert
			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_givenLocationFindCorrectOrderOfVaporize(t *testing.T) {
	data := []struct {
		fileName               string
		monitoringStationCoord Coordinate
		expected               int
	}{
		{
			"day10",
			Coordinate{19, 14},
			305,
		},
		{
			"day10_test2",
			Coordinate{11, 13},
			802,
		},
	}
	for _, d := range data {
		t.Run(d.fileName, func(t *testing.T) {
			// Arrange
			data := read.ReadDataAsString(d.fileName)
			asteroidMap := newAsteroidMap(data)
			monitoringStation := newMonitoringStation(asteroidMap)

			// Act
			vaporized := monitoringStation.vaporizeAsteroids(d.monitoringStationCoord)
			actual := vaporized[199].X*100 + vaporized[199].Y
			// Assert

			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}

}
