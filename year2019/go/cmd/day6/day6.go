package main

import (
	"fmt"
	"os"
	"strings"
)

type orbitCalculator struct {
	orbits map[string][]string
}

type orbitMinimalDistanceCalculator struct {
	orbitCalculator
}

func newOrbitMinimalDistanceCalculator(lines []string) *orbitMinimalDistanceCalculator {
	orbits := map[string][]string{}
	for _, line := range lines {
		split := strings.Split(line, ")")
		if len(split) != 2 {
			panic(fmt.Sprintf("Split not ending in two centers %s", split))
		}
		orbits[split[0]] = append(orbits[split[0]], split[1])
		orbits[split[1]] = append(orbits[split[1]], split[0])
	}
	return &orbitMinimalDistanceCalculator{
		orbitCalculator: orbitCalculator{
			orbits: orbits,
		},
	}
}

type visited map[string]struct{}

func (v *visited) add(toAdd string) {
	(*v)[toAdd] = struct{}{}
}

func (v *visited) contains(value string) bool {
	if _, ok := (*v)[value]; ok {
		return true
	}
	return false
}

func (omdc *orbitMinimalDistanceCalculator) getMinimalDistance(from string, to string) (int, error) {

	visited := visited{}
	found, findDistance, err := omdc.LookInNeighborOrbits(from, to, -1, visited)
	if err != nil {
		return 0, fmt.Errorf("not able to find minimal distance due to error: %w", err)
	}
	if !found {
		return 0, fmt.Errorf("not able to find minimal distance between %s and %s", from, to)
	}
	return findDistance, nil
}

func (omdc *orbitMinimalDistanceCalculator) LookInNeighborOrbits(from string, to string, debt int, visited visited) (bool, int, error) {
	visited.add(from)
	around, ok := omdc.orbits[from]
	if !ok {
		return false, 0, fmt.Errorf("not able to get center %s", from)
	}
	for _, o := range around {
		if visited.contains(o) {
			continue
		}
		if o == to {
			return true, debt, nil
		}
		found, neighborLookup, err := omdc.LookInNeighborOrbits(o, to, debt+1, visited)
		if err != nil {
			return false, 0, fmt.Errorf("error when visiting neighbor: %w", err)
		}
		if found {
			return found, neighborLookup, nil
		}
	}
	return false, 0, nil
}

type orbitCountCalculator struct {
	orbitCalculator
}

func newOrbitCountCalculator(lines []string) *orbitCountCalculator {
	orbits := map[string][]string{}
	for _, line := range lines {
		split := strings.Split(line, ")")
		if len(split) != 2 {
			panic(fmt.Sprintf("split not ending in two centers %s", split))
		}
		orbits[split[0]] = append(orbits[split[0]], split[1])
	}
	return &orbitCountCalculator{
		orbitCalculator: orbitCalculator{
			orbits: orbits,
		},
	}
}

func (occ *orbitCountCalculator) getOrbitCount() int {
	return occ.getAroundOrbits("COM", 1)
}

func (occ *orbitCountCalculator) getAroundOrbits(center string, debt int) int {
	around, ok := occ.orbits[center]
	if !ok {
		return 0
	}
	sum := len(around) * debt
	for _, o := range around {
		sum += occ.getAroundOrbits(o, debt+1)
	}
	return sum
}

func readData() []string {
	f, err := os.ReadFile("../../../data/day6_data.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), "\r\n")
}
