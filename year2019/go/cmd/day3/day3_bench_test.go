package main

import (
	"fmt"
	"math/rand"
	"testing"
)

var blackhole_getIntersection []coordinate

func Benchmark_getIntersection(b *testing.B) {
	for _, v := range []int{1, 10, 100, 1_000, 10_000} {
		coordinatesOne := createRandomCoordinateVisits(v)
		coordinatesTwo := createRandomCoordinateVisits(v)
		b.Run(fmt.Sprintf("Size static-%d", v), func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				blackhole_getIntersection = getIntersection(coordinatesOne, coordinatesTwo)
			}
		})
		b.Run(fmt.Sprintf("Size generic-%d", v), func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				blackhole_getIntersection = getIntersectionGeneric(coordinatesOne, coordinatesTwo)
			}
		})
	}
}

func createRandomCoordinateVisits(size int) coordinateVisits {
	coordinateVisits := coordinateVisits{}
	for {
		coordinateVisits[coordinate{rand.Intn(size), rand.Intn(size)}] = 0
		if len(coordinateVisits) >= size {
			break
		}
	}
	return coordinateVisits
}
