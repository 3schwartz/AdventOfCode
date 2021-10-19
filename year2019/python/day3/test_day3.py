import unittest

from year2019.python.day3.day3_func import *


class TestDay3(unittest.TestCase):

    def test_correct_distance_min(self):
        # Arrange
        first = ['R75', 'D30', 'R83', 'U83', 'L12', 'D49', 'R71', 'U7', 'L72']
        second = ['U62', 'R66', 'U55', 'R34', 'D71', 'R55', 'D58', 'R83']
        expected = 159

        # Act
        actual = get_minimum_distance(first, second)

        # Assert
        self.assertEqual(expected, actual)

    def test_correct_distance_min2(self):
        # Arrange
        first = ['R8', 'U5', 'L5', 'D3']
        second = ['U7', 'R6', 'D4', 'L4']
        expected = 6

        # Act
        actual = get_minimum_distance(first, second)

        # Assert
        self.assertEqual(expected, actual)

    def test_correct_steps(self):
        # Arrange
        first = ['R8', 'U5', 'L5', 'D3']
        second = ['U7', 'R6', 'D4', 'L4']
        expected = 30

        # Act
        actual = get_minimum_steps(first, second)

        # Assert
        self.assertEqual(expected, actual)
