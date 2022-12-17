package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("16")

	rates, graph := createRatesAndGraph(input)
	maxPressure := dfs(visit{
		state:         0,
		valve:         "AA",
		minute:        1,
		pressureCount: 0,
	}, 30, createLoopUp(graph), rates, graph)

	fmt.Printf("Part 1: %d\n", maxPressure)

	elephantPressure := dfs(visit{
		state:         0,
		valve:         "AA",
		minute:        1,
		pressureCount: 0,
		elephant:      true,
	}, 26, createLoopUp(graph), rates, graph)

	fmt.Printf("Part 2: %d\n", elephantPressure)
}

func createRatesAndGraph(input string) (map[string]int, map[string]map[string]int) {
	rates := map[string]int{}
	connections := map[string][]string{}
	for _, line := range strings.Split(input, "\r\n") {
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
		connections[valve] = destinations
		rates[valve] = flow
	}

	graph := map[string]map[string]int{}
	for key, value := range rates {
		if value == 0 && key != "AA" {
			continue
		}
		queue := collections.CreateQueue[queueElement]()
		for _, elm := range connections[key] {
			queue.Append(queueElement{valve: elm, steps: 1})
		}
		visited := map[string]struct{}{}
		destinations := map[string]int{}
		for queue.Len() > 0 {
			elm, ok := queue.TryDequeue()
			if !ok {
				break
			}
			if _, ok := visited[elm.valve]; ok {
				continue
			}
			visited[elm.valve] = struct{}{}
			if rates[elm.valve] > 0 {
				destinations[elm.valve] = elm.steps
				// continue
			}
			for _, n := range connections[elm.valve] {
				queue.Append(queueElement{n, elm.steps + 1})
			}
		}
		graph[key] = destinations
	}
	return rates, graph
}

func dfs(v visit, loopCount int, lu loopUp, rates map[string]int, graph map[string]map[string]int) int {
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
		temp := dfs(visit{
			state:         newState,
			valve:         key,
			minute:        time,
			pressureCount: v.pressureCount + rates[key]*(1+loopCount-time),
			elephant:      v.elephant,
		}, loopCount, lu, rates, graph)

		if temp > pressure {
			pressure = temp
		}
	}
	if v.elephant {
		temp := dfs(visit{
			state:         v.state,
			valve:         "AA",
			minute:        1,
			pressureCount: v.pressureCount,
			elephant:      false,
		}, loopCount, lu, rates, graph)
		if temp > pressure {
			pressure = temp
		}
	}
	return pressure
}

type visit struct {
	state         int
	valve         string
	minute        int
	pressureCount int
	elephant      bool
}

type loopUp map[string]int

func createLoopUp(graph map[string]map[string]int) loopUp {
	up := make(loopUp)
	count := 1
	for key := range graph {
		up[key] = count
		count++
	}
	return up
}

type queueElement struct {
	valve string
	steps int
}
