package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("19")
	blueprints := createBlueprints(input)

	sum := findSum(blueprints)
	fmt.Printf("Part 1: %d\n", sum)

	mul := findMul(blueprints, 3)
	fmt.Printf("Part 2: %d\n", mul)
}

func findMul(blueprints []blueprint, size int) int {
	mul := 1
	for _, b := range blueprints[:size] {
		optimal := dfs(b, 32, 1, 0, 0, 0, 0, 0, 0, 0, make(map[visit]int))
		mul *= optimal
	}
	return mul
}

func findSum(blueprints []blueprint) int {
	sum := 0
	for _, b := range blueprints {
		optimal := dfs(b, 24, 1, 0, 0, 0, 0, 0, 0, 0, make(map[visit]int))
		sum += b.id * optimal
	}
	return sum
}

type blueprint struct {
	id,
	oOre, cOre,
	obOre, obClay,
	gOre, gObsidian int
}

func createBlueprints(input string) []blueprint {
	lines := strings.Split(input, "\r\n")
	blueprints := make([]blueprint, len(lines))
	for i, line := range lines {
		costs := strings.Split(line, "costs ")
		id, err := strconv.Atoi(costs[0][10:(len(costs[0]) - 17)])
		if err != nil {
			panic(err)
		}
		o := strings.Split(costs[1], " ")
		on, err := strconv.Atoi(o[0])
		if err != nil {
			panic(err)
		}
		c := strings.Split(costs[2], " ")
		cn, err := strconv.Atoi(c[0])
		if err != nil {
			panic(err)
		}
		ob := strings.Split(costs[3], " ")
		obo, err := strconv.Atoi(ob[0])
		if err != nil {
			panic(err)
		}
		obc, err := strconv.Atoi(ob[3])
		if err != nil {
			panic(err)
		}
		g := strings.Split(costs[4], " ")
		gno, err := strconv.Atoi(g[0])
		if err != nil {
			panic(err)
		}
		gnb, err := strconv.Atoi(g[3])
		if err != nil {
			panic(err)
		}
		blueprints[i] = blueprint{
			id:   id,
			oOre: on, cOre: cn,
			obOre: obo, obClay: obc,
			gOre: gno, gObsidian: gnb,
		}
	}
	return blueprints
}

type visit struct {
	time, or, o, cr, c, obr, ob, gr, g int
}

func dfs(bp blueprint, time, or, o, cr, c, obr, ob, gr, g int, visited map[visit]int) int {
	var optimal int
	if time == 0 {
		return g
	}
	if oMax := max(bp.cOre, bp.gOre, bp.obOre, bp.oOre)*time - or*(time-1); o > oMax {
		o = oMax
	}
	if cMax := bp.obClay*time - cr*(time-1); c > cMax {
		c = cMax
	}
	if obMax := bp.gObsidian*time - obr*(time-1); ob > obMax {
		ob = obMax
	}
	v := visit{time, or, o, cr, c, obr, ob, gr, g}
	if opt, ok := visited[v]; ok {
		return opt
	}

	if bp.gOre <= o && bp.gObsidian <= ob {
		// Bue Geode
		optimal = max(dfs(bp, time-1, or, o+or-bp.gOre, cr, c+cr, obr, ob+obr-bp.gObsidian, gr+1, g+gr, visited), optimal)

	}
	if bp.obOre <= o && bp.obClay <= c && obr < bp.gObsidian {
		// Buy Obsidian
		optimal = max(dfs(bp, time-1, or, o+or-bp.obOre, cr, c+cr-bp.obClay, obr+1, ob+obr, gr, g+gr, visited), optimal)

	}
	if bp.cOre <= o && cr < bp.obClay {
		// Buy Clay
		optimal = max(dfs(bp, time-1, or, o+or-bp.cOre, cr+1, c+cr, obr, ob+obr, gr, g+gr, visited), optimal)

	}
	if bp.oOre <= o && or < max(bp.cOre, bp.gOre, bp.obOre) {
		// Buy Ore
		optimal = max(dfs(bp, time-1, or+1, o+or-bp.oOre, cr, c+cr, obr, ob+obr, gr, g+gr, visited), optimal)
	}

	optimal = max(dfs(bp, time-1, or, o+or, cr, c+cr, obr, ob+obr, gr, g+gr, visited), optimal)
	visited[v] = optimal
	return optimal
}

func max(f ...int) int {
	max := f[0]
	for _, i := range f {
		if i > max {
			max = i
		}
	}
	return max
}
