import unittest
from day4_func import *


class TestDay4(unittest.TestCase):
    def test_when_given_range_then_return_correct_count(self):
        # Arrange
        passwords = [12344, 12345]
        validator = PasswordValidator()
        validations = [TwoSequentuallyEqual(), IncreasingValidation()]
        validator.validations = validations

        # Act
        numberValidPasswords = validator.numberValidPassword(passwords)

        # Assert
        self.assertEqual(numberValidPasswords, 1)

    def test_wrong_password_validate_false(self):
        # Arrange
        password = 1234
        validator = PasswordValidator()
        validations = [TwoSequentuallyEqual(), IncreasingValidation()]
        validator.validations = validations

        # Act
        valid = validator.validatePassword(password)

        # Assert
        self.assertEqual(valid, False)

    def test_correct_password_validate_true(self):
        # Arrange
        password = 1233
        validator = PasswordValidator()
        validations = [TwoSequentuallyEqual(), IncreasingValidation()]
        validator.validations = validations

        # Act
        valid = validator.validatePassword(password)

        # Assert
        self.assertEqual(valid, True)

    def test_two_equal_false(self):
        # Arrange
        password = "1234"
        validation = TwoEqual()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, False)

    def test_two_equal_correct(self):
        # Arrange
        password = "1123"
        validation = TwoEqual()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, True)

    def test_two_sequentually_equal_correct(self):
        # Arrange
        password = "1123"
        validation = TwoSequentuallyEqual()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, True)

    def test_two_sequentually_equal_false(self):
        # Arrange
        password = "1234"
        validation = TwoSequentuallyEqual()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, False)

    def test_increasing_validation_not_all(self):
        # Arrange
        password = "1213"
        validation = IncreasingValidation()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, False)

    def test_increasing_validation_all_decrease(self):
        # Arrange
        password = "4321"
        validation = IncreasingValidation()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, False)

    def test_increasing_validation_correct(self):
        # Arrange
        password = "1234"
        validation = IncreasingValidation()

        # Act
        valid = validation.validate(password)

        # Assert
        self.assertEqual(valid, True)


    def test_password_finder_correct_range(self):
        # Arrange
        start = 1
        end = 3
        passwordFinder = PasswordFinder(start, end)

        # Act
        passwords = passwordFinder.getPasswords()

        # Assert
        self.assertEqual(len(passwords), 3)
        self.assertEqual(passwords[0], 1)
        self.assertEqual(passwords[1], 2)
        self.assertEqual(passwords[2], 3)