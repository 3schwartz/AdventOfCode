import unittest

from year2019.day1.day1_func import get_fuel


class TestDay1(unittest.TestCase):

    def test_correct_fuel_12(self):
        # Arrange
        expected = 2

        # Act
        actual = get_fuel(12)

        # Assert
        self.assertEqual(actual, expected)

    def test_correct_fuel_100756(self):
        # Arrange
        expected = 33583

        # Act
        actual = get_fuel(100756)

        # Assert
        self.assertEqual(actual, expected)

    def test_correct_fuel_14(self):
        # Arrange
        expected = 2

        # Act
        actual = get_fuel(14)

        # Assert
        self.assertEqual(actual, expected)

    def test_correct_fuel_1969(self):
        # Arrange
        expected = 654

        # Act
        actual = get_fuel(1969)

        # Assert
        self.assertEqual(actual, expected)
