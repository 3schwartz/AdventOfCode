from day4_func import *

passwordFinder = PasswordFinder(272091, 815432)
passwords = passwordFinder.getPasswords()

validator = PasswordValidator()
validations = [TwoSequentuallyEqual(), IncreasingValidation()]

validator.validations = validations

numberValidPasswords = validator.numberValidPassword(passwords)

print(f"Part 1: {numberValidPasswords}")

validations.append(TwoEqual())

validator.validations = validations

numberValidPasswordsWithTwoEqual = validator.numberValidPassword(passwords)

print(f"Part 2: {numberValidPasswordsWithTwoEqual}")