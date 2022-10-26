package main

import (
	"fmt"
	"math/big"
	"os"
	"strconv"
	"strings"
)

var cutSize int
var incSize int

func init() {
	cutSize = len("cut ")
	incSize = len("deal with increment ")
}

func main() {
	lines := parseData("day22_data")
	shuffler := deckShuffler{}

	output := shuffler.iterateLines(lines, 10007)

	fmt.Printf("Part 1: %d\n", shuffler.findIndex(output, 2019))

	moduloShuffler := createDeckModuloShuffler(lines, 119315717514047)
	moduloShuffler = moduloShuffler.shuffle(101741582076661)

	fmt.Printf("Part 2: %d", moduloShuffler.findIndex(2020))
}

func parseData(fileName string) []string {
	f, err := os.ReadFile(fmt.Sprintf("../../../data/%s.txt", fileName))
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(f), "\r\n")

	return lines
}

type deckModuloShuffler struct {
	helper    cardShufflerHelper
	offset    int
	increment int
	deckSize  int
}

func (dms deckModuloShuffler) findIndex(index int) int {
	return dms.helper.modulo(dms.offset+index*dms.increment, dms.deckSize)
}

func (dms deckModuloShuffler) shuffle(shuffleAmount int) deckModuloShuffler {
	increment := dms.helper.moduloInverse(dms.increment, shuffleAmount, dms.deckSize)

	incrementInverse := (1 - dms.increment) % dms.deckSize
	incrementInverse = dms.helper.moduloInverse(incrementInverse, dms.deckSize-2, dms.deckSize)

	offset := dms.calculateFinalGeometricOffset(dms.offset, 1-increment, incrementInverse, dms.deckSize)

	return deckModuloShuffler{
		offset:    offset,
		increment: increment,
		deckSize:  dms.deckSize,
	}
}

func createDeckModuloShuffler(lines []string, deckSize int) deckModuloShuffler {
	dms := deckModuloShuffler{}

	increment, offset := 1, 0
	for _, line := range lines {
		if strings.Contains(line, "cut") {
			cutInput, err := strconv.Atoi(line[cutSize:])
			if err != nil {
				panic(err)
			}
			offset += cutInput * increment
			offset = dms.helper.modulo(offset, deckSize)
			continue
		}
		if strings.Contains(line, "deal with increment") {
			incInput, err := strconv.Atoi(line[incSize:])
			if err != nil {
				panic(err)
			}
			inverse := dms.helper.moduloInverse(incInput, deckSize-2, deckSize)
			increment = dms.helper.multiplyThenModulo(increment, inverse, deckSize)
			continue
		}
		if strings.Contains(line, "deal into new stack") {
			increment *= -1
			increment = dms.helper.modulo(increment, deckSize)
			offset += increment
			offset = dms.helper.modulo(offset, deckSize)
			continue
		}
		panic(line)
	}
	dms.increment = increment
	dms.offset = offset
	dms.deckSize = deckSize
	return dms
}

func (dms deckModuloShuffler) calculateFinalGeometricOffset(a, b, c, mod int) int {
	x := new(big.Int)
	x.SetInt64(int64(a))

	y := new(big.Int)
	y.SetInt64(int64(b))

	z := new(big.Int)
	z.SetInt64(int64(c))

	res := new(big.Int)
	res.Mul(x, y)
	res.Mul(res, z)

	cards := new(big.Int)
	cards.SetInt64(int64(mod))
	modRes := new(big.Int)
	modRes.Mod(res, cards)

	return int(modRes.Int64())
}

type deckShuffler struct {
	helper cardShufflerHelper
}

func (d deckShuffler) findIndex(deck []int, valueAtIndex int) int {
	for i, card := range deck {
		if card == valueAtIndex {
			return i
		}
	}
	return 0
}

func (d deckShuffler) iterateLines(lines []string, size int) []int {
	input := d.helper.initializeArray(size)

	for _, line := range lines {
		if strings.Contains(line, "cut") {
			cutInput, err := strconv.Atoi(line[cutSize:])
			if err != nil {
				panic(err)
			}
			input = d.cut(input, cutInput)
			continue
		}
		if strings.Contains(line, "deal with increment") {
			incInput, err := strconv.Atoi(line[incSize:])
			if err != nil {
				panic(err)
			}
			input = d.increment(input, incInput)
			continue
		}
		if strings.Contains(line, "deal into new stack") {
			input = d.stack(input)
			continue
		}
		panic(line)
	}
	return input
}

func (d deckShuffler) stack(input []int) []int {
	output := make([]int, len(input))
	shift := len(input) - 1
	for i := 0; i < len(input); i++ {
		idx := -1*i + shift
		output[idx] = input[i]
	}
	return output
}

func (d deckShuffler) increment(input []int, inc int) []int {
	length := len(input)
	output := make([]int, length)
	idx := 0
	for i := 0; i < length; i++ {
		output[idx] = input[i]
		idx = d.helper.modulo(idx+inc, length)
	}
	return output
}

func (d deckShuffler) cut(input []int, cut int) []int {
	length := len(input)
	output := make([]int, length)
	shift := cut
	for i := 0; i < length; i++ {
		idx := d.helper.modulo(i-shift, length)
		output[idx] = input[i]
	}
	return output
}

type cardShufflerHelper struct{}

func (csh cardShufflerHelper) initializeArray(size int) []int {
	input := make([]int, size)
	for i := 0; i < size; i++ {
		input[i] = i
	}
	return input
}

func (csh cardShufflerHelper) modulo(in int, mod int) int {
	return (in%mod + mod) % mod
}

func (csh cardShufflerHelper) multiplyThenModulo(a, b, mod int) int {
	res := 0
	for b > 0 {
		if csh.modulo(b, 2) == 1 {
			res = csh.modulo(res+a, mod)
		}
		a = csh.modulo(a*2, mod)
		b = b / 2
	}
	return res
}

func (csh cardShufflerHelper) moduloInverse(x int, y int, m int) int {
	xBig := new(big.Int)
	xBig.SetInt64(int64(x))

	yBig := new(big.Int)
	yBig.SetInt64(int64(y))

	mBig := new(big.Int)
	mBig.SetInt64(int64(m))

	zBig := new(big.Int)
	zBig.Exp(xBig, yBig, mBig)

	return int(zBig.Int64())
}
