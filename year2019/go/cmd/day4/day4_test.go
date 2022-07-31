package main

import (
	"strconv"
	"testing"
	"unsafe"

	"github.com/google/go-cmp/cmp"
)

type validator func(password int) bool

func increasingValidator(password int) bool {
	pass := strconv.Itoa(password)
	valid := true
	for i := 0; i < len(pass)-1; i++ {
		// Benchmark below conversion
		// first := int8(pass[i] - '0')
		// second := int8(pass[i+1] - '0')
		firstByte := pass[i] - '0'
		first := *(*int8)(unsafe.Pointer(&firstByte))
		secondByte := pass[i+1] - '0'
		second := *(*int8)(unsafe.Pointer(&secondByte))
		valid = valid && first <= second
	}
	return valid
}

func twoSequentiallyEqualValidator(password int) bool {
	pass := strconv.Itoa(password)
	valid := false
	for i := 0; i < len(pass)-1; i++ {
		// Benchmark below conversion
		// first := int8(pass[i] - '0')
		// second := int8(pass[i+1] - '0')
		firstByte := pass[i] - '0'
		first := *(*int8)(unsafe.Pointer(&firstByte))
		secondByte := pass[i+1] - '0'
		second := *(*int8)(unsafe.Pointer(&secondByte))
		valid = valid || first == second
	}
	return valid
}

type passwordValidator struct {
	passwords []int
}

func newPasswordValidator(from int, to int) passwordValidator {
	size := 1 + to - from
	passwords := make([]int, 0, size)
	for i := from; i <= to; i++ {
		passwords = append(passwords, i)
	}
	return passwordValidator{
		passwords: passwords,
	}
}

func (pv *passwordValidator) getNumberValidPassword(validators []validator) int {
	sum := 0
	for _, password := range pv.passwords {
		valid := true
		for _, validator := range validators {
			valid = valid && validator(password)
		}
		if valid {
			sum += 1
		}
	}
	return sum
}

func Test_increasingValidator(t *testing.T) {
	data := []struct {
		name     string
		password int
		expected bool
	}{
		{"First",
			123,
			true},
		{
			"Second",
			121,
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

func Test_twoSequentiallyEqualValidator(t *testing.T) {
	data := []struct {
		name     string
		password int
		expected bool
	}{
		{"First",
			112,
			true},
		{
			"Second",
			123,
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
