package main

import (
	"fmt"
	"math"
	"os"
)

const (
	zeroAsByte byte = '0'
	oneAsByte  byte = '1'
	twoAsByte  byte = '2'
)

func main() {
	data := readData()
	part2(data)
}

func part2(data string) {
	composedImage := newComposedImage(data)
	composedImage.printImage()
}

type pixelPoint struct {
	x int
	y int
}

type composedImage struct {
	columns int
	rows    int
	layer   map[pixelPoint]byte
}

func newComposedImage(data string) *composedImage {
	columns := 25
	rows := 6
	imageSize := rows * columns
	image := map[pixelPoint]byte{}
	for i := 0; i < len(data); i++ {
		row := i/columns - rows*(i/imageSize)
		column := i % columns
		if data[i] == twoAsByte {
			continue
		}
		pixel := pixelPoint{x: row, y: column}
		_, ok := image[pixel]
		if ok {
			continue
		}
		image[pixel] = data[i]
	}
	return &composedImage{
		layer:   image,
		columns: columns,
		rows:    rows,
	}
}

func (ci *composedImage) printImage() {
	rowsInput := make([]string, ci.columns)
	pixel := pixelPoint{}
	for i := 0; i < ci.rows; i++ {
		for j := 0; j < ci.columns; j++ {
			pixel.x = i
			pixel.y = j
			output := "."
			if ci.layer[pixel] == oneAsByte {
				output = "#"
			}
			rowsInput[j] = output
		}
		fmt.Println(rowsInput)
	}
}

func readData() string {
	f, err := os.ReadFile("../../../data/day8_data.txt")
	if err != nil {
		panic(err)
	}
	return string(f)
}

type image struct {
	layers map[int][]byte
}

func newImage(data string) *image {
	imageSize := 25 * 6
	layers := make(map[int][]byte, len(data)/imageSize)
	for i := 0; i < len(data); i++ {
		layers[i/imageSize] = append(layers[i/imageSize], data[i])
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
			if pixel == zeroAsByte {
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
		if pixel == oneAsByte {
			oneDigits++
		}
		if pixel == twoAsByte {
			twoDigits++
		}
	}
	return oneDigits, twoDigits
}
