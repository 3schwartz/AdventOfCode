package main

import (
	"advent/pkg/coders"
	"os"
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

var blackhole int

func Benchmark_coders(b *testing.B) {
	input := readData()
	codes := coders.ParseIntCodes(input)

	b.Run("ChannelCoder", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackhole = coders.ChannelCoderFindMaxThrusterSignal(codes, coders.FromTo{From: 0, To: 5})
		}
	})
	b.Run("ReactiveCoder", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackhole = coders.ReactiveCoderFindMaxThrusterSignal(codes, coders.FromTo{From: 0, To: 5})
		}
	})
}

func TestSolutions(t *testing.T) {
	data := []struct {
		name  string
		coder func([]int, coders.FromTo) int
	}{
		{
			"Channel",
			coders.ChannelCoderFindMaxThrusterSignal,
		},
		{
			"Reactive",
			coders.ReactiveCoderFindMaxThrusterSignal,
		},
	}
	// Arrange
	input := readData()
	codes := coders.ParseIntCodes(input)

	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			actualPart1 := d.coder(codes, coders.FromTo{From: 0, To: 5})
			actualPart2 := d.coder(codes, coders.FromTo{From: 5, To: 10})

			// Assert
			if diff := cmp.Diff(14902, actualPart1); diff != "" {
				t.Error(diff)
			}
			if diff := cmp.Diff(6489132, actualPart2); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func readData() []string {
	f, err := os.ReadFile("../../../data/day7_data.txt")
	if err != nil {
		panic(err)
	}
	return strings.Split(string(f), ",")
}

func Test_findMaxThrusterSignal(t *testing.T) {
	data := []struct {
		name     string
		codes    string
		coder    func([]int, coders.FromTo) int
		expected int
	}{
		{
			"channel-first",
			"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
			coders.ChannelCoderFindMaxThrusterSignal,
			139629729,
		},
		{
			"channel-second",
			"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
			coders.ChannelCoderFindMaxThrusterSignal,
			18216,
		},
		{
			"reactive-first",
			"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
			coders.ReactiveCoderFindMaxThrusterSignal,
			139629729,
		},
		{
			"reactive-second",
			"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
			coders.ReactiveCoderFindMaxThrusterSignal,
			18216,
		},
	}
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Arrange
			codes := coders.ParseIntCodes(strings.Split(d.codes, ","))

			// Act
			actual := d.coder(codes, coders.FromTo{From: 5, To: 10})

			// Assert
			if diff := cmp.Diff(d.expected, actual); diff != "" {
				t.Error(diff)
			}
		})
	}

}
