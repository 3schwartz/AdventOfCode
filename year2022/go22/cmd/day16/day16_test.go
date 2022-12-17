package main

import (
	"advent2022/pkg/io"
	"strconv"
	"strings"
	"testing"
)

func Test_part2(t *testing.T) {
	// Arrange
	input := io.ReadData("16_test")

	// Act
	rates, graph := createRatesAndGraph(input)
	maxPressure := dfs(visit{
		state:         0,
		valve:         "AA",
		minute:        1,
		pressureCount: 0,
		elephant:      true,
	}, 26, createLoopUp(graph), rates, graph)

	// Assert
	if maxPressure != 1707 {
		t.Error(maxPressure)
	}
}

func Test_part1(t *testing.T) {
	// Arrange
	input := io.ReadData("16_test")

	// Act
	rates, graph := createRatesAndGraph(input)
	maxPressure := dfs(visit{
		state:         0,
		valve:         "AA",
		minute:        1,
		pressureCount: 0,
	}, 30, createLoopUp(graph), rates, graph)

	// Assert
	if maxPressure != 1651 {
		t.Error(maxPressure)
	}
}

func Test_part2_cache(t *testing.T) {
	// Arrange
	input := io.ReadData("16_test")

	// Act
	rates, graph := createRatesAndGraph(input)
	cache := make(map[visit]int)
	maxPressure := dfsCache(visit{
		state:         0,
		valve:         "AA",
		minute:        1,
		pressureCount: 0,
		elephant:      true,
	}, 26, createLoopUp(graph), rates, graph, cache)

	// Assert
	if maxPressure != 1707 {
		t.Error(maxPressure)
	}
}

func Test_part2_value(t *testing.T) {
	// Arrange
	input := io.ReadData("16_test")

	// Act
	rates, graph := createRatesAndGraph(input)
	maxPressure := dfsValue(0, "AA", 1, true, 26, createLoopUp(graph), rates, graph)

	// Assert
	if maxPressure != 1707 {
		t.Error(maxPressure)
	}
}

func Test_part2_array(t *testing.T) {
	// Arrange
	input := io.ReadData("16_test")

	// Act
	maxPressure := usingArray(input, 26, true)

	// Assert
	if maxPressure != 1707 {
		t.Error(maxPressure)
	}
}

var blackhole int

func Benchmark_part2(b *testing.B) {
	data := "16"
	b.Run("Value", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			input := io.ReadData(data)
			rates, graph := createRatesAndGraph(input)
			blackhole = dfsValue(0, "AA", 1, true, 26, createLoopUp(graph), rates, graph)
		}
	})
	b.Run("Cache", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			input := io.ReadData(data)
			rates, graph := createRatesAndGraph(input)
			cache := make(map[visit]int)
			blackhole = dfsCache(visit{
				state:         0,
				valve:         "AA",
				minute:        1,
				pressureCount: 0,
				elephant:      true,
			}, 26, createLoopUp(graph), rates, graph, cache)
		}
	})
	b.Run("Default", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			input := io.ReadData(data)
			rates, graph := createRatesAndGraph(input)
			blackhole = dfs(visit{
				state:         0,
				valve:         "AA",
				minute:        1,
				pressureCount: 0,
				elephant:      true,
			}, 26, createLoopUp(graph), rates, graph)
		}
	})
	b.Run("Array", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			input := io.ReadData(data)
			blackhole = usingArray(input, 26, true)
		}
	})
}

func dfsValue(state int, valve string, time int, elephant bool,
	loopCount int, lu loopUp, rates map[string]int, graph map[string]map[string]int) int {
	var pressure int
	if time == loopCount && !elephant {
		return pressure
	}
	for key, value := range graph[valve] {
		newState := state | (1 << lu[key])
		if newState == state {
			continue
		}
		newTime := time + 1 + value
		if newTime > loopCount {
			continue
		}
		temp := rates[key]*(1+loopCount-newTime) +
			dfsValue(newState, key, newTime, elephant,
				loopCount, lu, rates, graph)

		if temp > pressure {
			pressure = temp
		}
	}
	if elephant && pressure != 0 {
		temp := dfsValue(state, "AA", 1, false,
			loopCount, lu, rates, graph)
		if temp > pressure {
			pressure = temp
		}
	}
	return pressure
}

func dfsCache(v visit, loopCount int, lu loopUp, rates map[string]int, graph map[string]map[string]int, cache map[visit]int) int {
	if cachePressure, ok := cache[v]; ok {
		return cachePressure
	}
	pressure := v.pressureCount
	if v.minute == loopCount && !v.elephant {
		return pressure
	}
	for key, value := range graph[v.valve] {
		newState := v.state | (1 << lu[key])
		if newState == v.state {
			continue
		}
		time := v.minute + 1 + value
		if time > loopCount {
			continue
		}
		temp := dfsCache(visit{
			state:         newState,
			valve:         key,
			minute:        time,
			pressureCount: v.pressureCount + rates[key]*(1+loopCount-time),
			elephant:      v.elephant,
		}, loopCount, lu, rates, graph, cache)

		if temp > pressure {
			pressure = temp
		}
	}
	if v.elephant && pressure != v.pressureCount {
		temp := dfsCache(visit{
			state:         v.state,
			valve:         "AA",
			minute:        1,
			pressureCount: v.pressureCount,
			elephant:      false,
		}, loopCount, lu, rates, graph, cache)
		if temp > pressure {
			pressure = temp
		}
	}
	cache[v] = pressure
	return pressure
}

func usingArray(input string, loop int, elephant bool) int {
	rates := make([]int, 0)
	// valves := make([]string, 0)
	valvesLookup := map[string]int{}
	connections := make([][]string, 0)
	for i, line := range strings.Split(input, "\r\n") {
		split := strings.Split(line, "; tunnels lead to valves ")
		if len(split) == 1 {
			split = strings.Split(line, "; tunnel leads to valve ")
		}
		valve := split[0][6:8]
		flow, err := strconv.Atoi(split[0][23:])
		if err != nil {
			panic(err)
		}
		destinations := strings.Split(split[1], ", ")

		// valves = append(valves, valve)
		valvesLookup[valve] = i
		connections = append(connections, destinations)
		rates = append(rates, flow)
	}
	distances := make([][]int, len(rates))
	for i := 0; i < len(rates); i++ {
		jLine := make([]int, len(rates))
		for j := 0; j < len(rates); j++ {
			jLine[j] = 100
		}
		distances[i] = jLine
	}
	for i := 0; i < len(connections); i++ {
		for _, connection := range connections[i] {
			distances[i][valvesLookup[connection]] = 1
		}
	}
	// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
	for k := 0; k < len(rates); k++ {
		for i := 0; i < len(rates); i++ {
			for j := 0; j < len(rates); j++ {
				temp := distances[i][k] + distances[k][j]
				if temp < distances[i][j] {
					distances[i][j] = temp
				}
			}
		}
	}
	idxAboveZero := make([]int, 0)
	for i, flow := range rates {
		if flow > 0 {
			idxAboveZero = append(idxAboveZero, i)
		}
	}
	start := valvesLookup["AA"]
	final := dfsArray(start, loop, start, loop, idxAboveZero, distances, rates, elephant)

	return final
}

func dfsArray(start, startTime, this, time int, missingVisit []int, distances [][]int,
	rates []int, elephant bool) int {

	var pressure int
	for _, c := range missingVisit {
		if distances[this][c] >= time {
			continue
		}
		toVisit := missing(c, missingVisit)
		newTime := (time - distances[this][c] - 1)
		temp := rates[c]*newTime +
			dfsArray(start, startTime, c, newTime, toVisit, distances, rates, elephant)
		if temp > pressure {
			pressure = temp
		}
	}
	if elephant {
		temp := dfsArray(start, startTime, start, startTime, missingVisit,
			distances, rates, false)
		if temp > pressure {
			pressure = temp
		}
	}
	return pressure
}

func missing(i int, old []int) []int {
	new := make([]int, len(old)-1)
	idx := 0
	for _, c := range old {
		if c == i {
			continue
		}
		new[idx] = c
		idx++
	}
	return new
}
