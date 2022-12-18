package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"math"
	"strconv"
	"strings"
)

var neighbors [9]coord

func init() {
	neighbors = [9]coord{
		{0, 0, 1}, {0, 1, 0}, {1, 0, 0},
		{0, 0, -1}, {0, -1, 0}, {-1, 0, 0},
	}
}

func main() {
	input := io.ReadData("18")
	eKube, mm := createKube(input)

	surface := eKube.findSurface()
	fmt.Printf("Part 1: %d\n", surface)

	exteriorSurface := eKube.findExteriorSurface(mm)
	fmt.Printf("Part 2: %d\n", exteriorSurface)
}

type coord struct {
	x, y, z int
}

func (c coord) add(other coord) coord {
	return coord{c.x + other.x, c.y + other.y, c.z + other.z}
}

type minMax struct {
	xMin, xMax, yMin, yMax, zMin, zMax int
}

func initMinMax() minMax {
	return minMax{
		xMin: math.MaxInt,
		yMin: math.MaxInt,
		zMin: math.MaxInt,
		xMax: math.MinInt,
		yMax: math.MinInt,
		zMax: math.MinInt,
	}
}

func (mm minMax) isOutside(cur coord) bool {
	return cur.x < mm.xMin || cur.x > mm.xMax ||
		cur.y < mm.yMin || cur.y > mm.yMax ||
		cur.z < mm.zMin || cur.z > mm.zMax
}

type kube map[coord]struct{}

func createKube(input string) (kube, minMax) {
	k := make(kube)
	mm := initMinMax()
	for _, line := range strings.Split(input, "\r\n") {
		c := coord{}
		for i, split := range strings.Split(line, ",") {
			n, err := strconv.Atoi(split)
			if err != nil {
				panic(err)
			}
			switch i {
			case 0:
				c.x = n
				if n < mm.xMin {
					mm.xMin = n
				}
				if n > mm.xMax {
					mm.xMax = n
				}
			case 1:
				c.y = n
				if n < mm.yMin {
					mm.yMin = n
				}
				if n > mm.yMax {
					mm.yMax = n
				}
			case 2:
				c.z = n
				if n < mm.zMin {
					mm.zMin = n
				}
				if n > mm.zMax {
					mm.zMax = n
				}
			}
		}
		k[c] = struct{}{}
	}
	return k, mm
}

func (k kube) findExteriorSurface(mm minMax) int {
	dense := k.findDense(mm)
	surface := 0
	for c := range k {
		for _, n := range neighbors {
			move := c.add(n)
			if _, ok := dense[move]; ok {
				continue
			}
			surface++
		}
	}
	return surface
}

func (k kube) findDense(mm minMax) kube {
	dense := make(kube)
	visited := make(kube)
	for c := range k {
		dense[c] = struct{}{}
		for _, n := range neighbors {
			move := c.add(n)
			if _, ok := k[move]; ok {
				continue
			}
			if _, ok := visited[move]; ok {
				continue
			}
			tempVisited := make(kube)
			queue := collections.CreateQueue[coord]()
			queue.Append(move)
			isWithin := true
			for queue.Len() > 0 {
				cur, _ := queue.TryDequeue()
				if _, v := visited[cur]; v {
					continue
				}
				if mm.isOutside(cur) {
					isWithin = false
					continue
				}
				if _, ok := k[cur]; ok {
					continue
				}
				tempVisited[cur] = struct{}{}
				visited[cur] = struct{}{}
				for _, n := range neighbors {
					cn := cur.add(n)
					queue.Append(cn)
				}
			}
			if !isWithin {
				continue
			}
			for tc := range tempVisited {
				dense[tc] = struct{}{}
			}
		}
	}
	return dense
}

func (k kube) findSurface() int {
	surface := 0
	for c := range k {
		for _, n := range neighbors {
			move := c.add(n)
			if _, ok := k[move]; ok {
				continue
			}
			surface++
		}
	}
	return surface
}
