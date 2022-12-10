package main

import (
	"advent2022/pkg/collections"
	"advent2022/pkg/io"
	"fmt"
)

func main() {
	input := io.ReadData("6")
	lastSequence := sequence{}
	for i, signal := range input {
		lastSequence = lastSequence.incSequence(signal)
		if i > 2 && lastSequence.isUnique() {
			fmt.Printf("Part 1: %d\n", i+1)
			break
		}
	}

	lastMessage := message{}
	inputs := collections.CreateQueue[rune]()
	for i, signal := range input {
		lastMessage.add(signal)
		inputs.Append(signal)
		if i < 13 {
			continue
		}
		if len(lastMessage) == 14 {
			fmt.Printf("Part 2: %d\n", i+1)
		}
		toRemove, ok := inputs.TryDequeue()
		if !ok {
			panic("not able to remove")
		}
		lastMessage.remove(toRemove)
	}
}

type message map[rune]int

func (m message) add(new rune) {
	m[new] = m[new] + 1
}

func (m message) remove(new rune) {
	old, ok := m[new]
	if !ok {
		return
	}
	if old == 1 {
		delete(m, new)
		return
	}
	m[new] = old - 1
}

type sequence struct {
	first  rune
	second rune
	third  rune
	fourth rune
}

func (s sequence) isUnique() bool {
	return s.first != s.second && s.first != s.third && s.first != s.fourth &&
		s.second != s.third && s.second != s.fourth &&
		s.third != s.fourth
}

func (s sequence) incSequence(new rune) sequence {
	return sequence{
		first:  s.second,
		second: s.third,
		third:  s.fourth,
		fourth: new,
	}
}
