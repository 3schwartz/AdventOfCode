package main

import (
	"advent/pkg/coders"
	"advent/pkg/read"
	"fmt"
)

func main() {
	codes := read.ReadData("day19")
	intCodes := coders.ParseIntCodes(codes)
	intCoder := coders.TractorBeamIntCoder{}
	ch := make(chan coders.BeamPoint)
	go intCoder.FindPointsAffected(intCodes, ch, 50)

	var sum int
	for i := range ch {
		sum += i.Pulled
	}
	fmt.Printf("Part 1: %d\n", sum)

	pulled := map[coders.BeamPoint]struct{}{}

	initPoint := coders.Coord{X: 0, Y: 0}
	toEvaluate := map[coders.Coord]struct{}{}
	toEvaluate[initPoint] = struct{}{}

	evaluated := map[coders.Coord]struct{}{}

	var beamInitFound bool

	for {
		nextToEvaluate := make(map[coders.Coord]struct{})
		chNeighbor := make(chan coders.BeamPoint, len(toEvaluate))
		go intCoder.FindNeighborsAffected(intCodes, toEvaluate, chNeighbor)
		for i := range chNeighbor {
			if i.Pulled == 0 && beamInitFound {
				continue
			}
			if i.X != 0 && i.Y != 0 && i.Pulled == 1 {
				beamInitFound = true
			}

			for nextNeighbor := range i.FindNeighbors() {
				_, ok := evaluated[nextNeighbor]
				if ok {
					continue
				}
				evaluated[nextNeighbor] = struct{}{}
				nextToEvaluate[nextNeighbor] = struct{}{}
			}

			pulled[i] = struct{}{}
			found, x, y := containsShip(i, pulled)

			if found {
				part2 := x*10_000 + y
				fmt.Printf("Part 2: %d\n", part2)
				return
			}
		}
		toEvaluate = make(map[coders.Coord]struct{})
		for key, value := range nextToEvaluate {
			toEvaluate[key] = value
		}
	}
}

func containsShip(beamPoint coders.BeamPoint, beamMap map[coders.BeamPoint]struct{}) (found bool, xStart int, yStart int) {
	xStart = beamPoint.X - 100
	yStart = beamPoint.Y - 100
	for x := xStart; x < beamPoint.X; x++ {
		for y := yStart; y < beamPoint.Y; y++ {
			_, ok := beamMap[coders.BeamPoint{
				Coord:  coders.Coord{X: x, Y: y},
				Pulled: 1,
			}]
			if !ok {
				return false, 0, 0
			}
		}
	}
	return true, xStart, yStart
}
