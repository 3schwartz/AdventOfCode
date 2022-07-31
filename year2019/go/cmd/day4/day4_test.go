package main

import (
	"math/rand"
	"strconv"
	"testing"

	"github.com/google/go-cmp/cmp"
)

var blackholeFirst, blackholeSecond int8

func Benchmark_mapIntegersFromAsciiBytes(b *testing.B) {
	number := strconv.Itoa(rand.Intn(100) + 10)
	b.Run("Pointers", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackholeFirst, blackholeSecond = mapIntegersFromAsciiBytesUsingPointers(number[0], number[1])
		}
	})
	b.Run("Converter", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			blackholeFirst, blackholeSecond = mapIntegersFromAsciiBytesCastUsingPointers(number[0], number[1])
		}
	})
}

func Test_passwordFinder(t *testing.T) {
	data := []struct {
		name       string
		validators []validator
		expected   int
	}{
		{
			"Part1",
			[]validator{increasingValidator, twoSequentiallyEqualValidator},
			931,
		},
		{
			"Part2",
			[]validator{increasingValidator, twoSequentiallyEqualValidator, twoEqualValidator},
			609,
		},
	}
	// Arrange
	passwordValidator := newPasswordValidator(272091, 815432)
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			actual := passwordValidator.getNumberValidPassword(d.validators)

			if diff := cmp.Diff(actual, d.expected); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_increasingValidator(t *testing.T) {
	data := []struct {
		name     string
		password string
		expected bool
	}{
		{"First",
			"123",
			true},
		{
			"Second",
			"121",
			false,
		},
	}
	// Arrange
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			actual := increasingValidator(d.password)

			// Assert
			if diff := cmp.Diff(actual, d.expected); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_twoEqualValidator(t *testing.T) {
	data := []struct {
		name     string
		password string
		expected bool
	}{
		{"First",
			"121",
			true},
		{
			"Second",
			"123",
			false,
		},
	}
	// Arrange
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			actual := twoEqualValidator(d.password)

			// Assert
			if diff := cmp.Diff(actual, d.expected); diff != "" {
				t.Error(diff)
			}
		})
	}
}

func Test_twoSequentiallyEqualValidator(t *testing.T) {
	data := []struct {
		name     string
		password string
		expected bool
	}{
		{"First",
			"112",
			true},
		{
			"Second",
			"123",
			false,
		},
	}
	// Arrange
	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			// Act
			actual := twoSequentiallyEqualValidator(d.password)

			// Assert
			if diff := cmp.Diff(actual, d.expected); diff != "" {
				t.Error(diff)
			}
		})
	}
}
