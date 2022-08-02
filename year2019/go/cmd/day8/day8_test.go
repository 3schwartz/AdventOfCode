package main

import (
	"math"
	"os"
	"testing"
)

type image struct {
	layers map[int][]int8
}

func newImage(data string) *image {
	imageSize := 25 * 6
	layers := make(map[int][]int8, len(data)/imageSize)

	for i, v := range data {
		layers[i/imageSize] = append(layers[i/imageSize], int8(v-'0'))
	}
	return &image{
		layers: layers,
	}
}

func (i *image) findLayerWithFewestZeroDigits() int {
	minZeroDigit := math.MaxInt
	minZeroIdx := math.MaxInt
	for idx, layer := range i.layers {
		var zeroDigits int
		for _, pixel := range layer {
			if pixel == 0 {
				zeroDigits++
			}
		}
		if zeroDigits < minZeroDigit {
			minZeroDigit = zeroDigits
			minZeroIdx = idx
		}
	}
	return minZeroIdx
}

func (i *image) findOneAndTwoDigitsCountInLayer(idx int) (int, int) {
	oneDigits := 0
	twoDigits := 0
	for _, pixel := range i.layers[idx] {
		if pixel == 1 {
			oneDigits++
		}
		if pixel == 2 {
			twoDigits++
		}
	}
	return oneDigits, twoDigits
}

func Test_part1(t *testing.T) {
	// Arrange
	data := readData()
	image := newImage(data)

	// Act
	idx := image.findLayerWithFewestZeroDigits()
	oneCount, twoCount := image.findOneAndTwoDigitsCountInLayer(idx)

	// Assert
	if result := oneCount * twoCount; result != 2684 {
		t.Errorf("wrong result: %d with oneCount: %d and twoCount: %d", result, oneCount, twoCount)
	}
}

func readData() string {
	f, err := os.ReadFile("../../../data/day8_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}
