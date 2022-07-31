package main

import (
	"strconv"
	"unsafe"
)

// Deprecated: For fun and benchmark
func mapIntegersFromAsciiBytesUsingPointers(firstByte byte, secondByte byte) (int8, int8) {
	firstByte = firstByte - '0'
	first := *(*int8)(unsafe.Pointer(&firstByte))
	secondByte = secondByte - '0'
	second := *(*int8)(unsafe.Pointer(&secondByte))
	return first, second
}

func mapIntegersFromAsciiBytesCastUsingPointers(firstByte byte, secondByte byte) (int8, int8) {
	first := int8(firstByte - '0')
	second := int8(secondByte - '0')
	return first, second
}

type validator func(password string) bool

func increasingValidator(password string) bool {
	valid := true
	for i := 0; i < len(password)-1; i++ {
		first, second := mapIntegersFromAsciiBytesUsingPointers(password[i], password[i+1])
		valid = valid && first <= second
	}
	return valid
}

func twoSequentiallyEqualValidator(password string) bool {
	valid := false
	for i := 0; i < len(password)-1; i++ {
		first, second := mapIntegersFromAsciiBytesUsingPointers(password[i], password[i+1])
		valid = valid || first == second
	}
	return valid
}

func twoEqualValidator(password string) bool {
	numbers := map[byte]int{}
	for i := 0; i < len(password); i++ {
		numbers[password[i]] += 1
	}
	for _, value := range numbers {
		if value == 2 {
			return true
		}
	}
	return false
}

type passwordValidator struct {
	passwords []string
}

func newPasswordValidator(from int, to int) passwordValidator {
	size := 1 + to - from
	passwords := make([]string, 0, size)
	for i := from; i <= to; i++ {
		password := strconv.Itoa(i)
		passwords = append(passwords, password)
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
