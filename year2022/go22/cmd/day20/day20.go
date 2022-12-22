package main

import (
	"advent2022/pkg/io"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := io.ReadData("20")
	p, n := initPositions(input, 1)

	c := mix(p, n, 1)
	sum := findIdxAfterZero(c, 1_000) + findIdxAfterZero(c, 2_000) + findIdxAfterZero(c, 3_000)

	fmt.Printf("Part 1: %d\n", sum)

	p, n = initPositions(input, 811589153)
	c = mix(p, n, 10)
	sum = findIdxAfterZero(c, modulo(1_000, n)) +
		findIdxAfterZero(c, modulo(2_000, n)) +
		findIdxAfterZero(c, modulo(3_000, n))
	fmt.Printf("Part 2: %d\n", sum)
}

type position struct {
	initialIdx, move int
	left, right      *position
}

func initPositions(input string, decryptionKey int) (*position, int) {
	var first *position
	var last *position
	lines := strings.Split(input, "\r\n")
	for i, line := range lines {
		n, err := strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
		p := &position{initialIdx: i, move: n * decryptionKey}
		if last == nil {
			first = p
			last = p
			continue
		}
		p.left = last
		last.right = p
		last = p
	}
	first.left = last
	last.right = first
	return first, len(lines)
}

func modulo(a, mod int) int {
	return (a%mod + mod) % mod
}

func mix(p *position, n, mixCount int) *position {
	c := p
	for mc := 0; mc < mixCount; mc++ {
		for i := 0; i < n; i++ {
			for {
				if c.initialIdx != i {
					c = c.right
					continue
				}
				if c.move == 0 {
					break
				}
				left := c.left
				right := c.right
				left.right = right
				right.left = left
				count := modulo(c.move, n-1) - 1
				current := right
				// if c.move < 0 {
				// 	current = left.left
				// 	count = c.move + 1
				// }
				for count != 0 {
					// if c.move < 0 {
					// 	current = current.left
					// 	count++
					// 	continue
					// }
					current = current.right
					count--
					continue
				}
				newRight := current.right
				current.right = c
				c.right = newRight
				newRight.left = c
				c.left = current
				break
			}
		}
		fmt.Printf("Done: %d\n", mc)
	}
	return c
}

func findIdxAfterZero(p *position, count int) int {
	c := p
	for {
		if c.move != 0 {
			c = c.right
			continue
		}
		for count != 0 {
			c = c.right
			count--
		}
		break
	}
	return c.move
}
