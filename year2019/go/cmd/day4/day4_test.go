package main

import (
	"strconv"
	"testing"
	"unsafe"

	"github.com/google/go-cmp/cmp"
)

type validator func(password int) bool

// func increasingValidator(password int) bool {
// 	valid := true
// 	for {
// 		valid = valid && (password%10 <= (password/10)%10)
// 		password = password / 10
// 		if password < 10 {
// 			break
// 		}
// 	}
// 	return valid
// }

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
func Test_passwordFinder(t *testing.T) {
	// data := []struct {
	// 	name string
	// 	validators []validator
	// }
}

func Test_passFoo(t *testing.T) {

}
