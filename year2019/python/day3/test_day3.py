import unittest

from year2019.python.day3.day3_func import *


class TestDay3(unittest.TestCase):

    def test_wire_init_correct(self):
        # Arrange
        wires = ["R87", "I9"]

        # Act
        wire_aggregator = Wire(wires)

        # Assert
        self.assertEqual(2, len(wire_aggregator.moves))


    def test_move_init_correct(self):
        # Arrange
        direction = "R75"

        # Act
        wire = Move(direction)

        # Assert
        self.assertEqual("R", wire.direction)
        self.assertEqual(75, wire.steps)

    def test_correct_distance_min(self):
        # Arrange
        first = ['R75', 'D30', 'R83', 'U83', 'L12', 'D49', 'R71', 'U7', 'L72']
        second = ['U62', 'R66', 'U55', 'R34', 'D71', 'R55', 'D58', 'R83']
        expected = 159

        # Act
        minimum_calculator = MinimumCalculator(first, second)
        actual = minimum_calculator.get_minimum_distance()

        # Assert
        self.assertEqual(expected, actual)

    def test_correct_distance_min2(self):
        # Arrange
        first = ['R8', 'U5', 'L5', 'D3']
        second = ['U7', 'R6', 'D4', 'L4']
        expected = 6

        # Act
        actual = MinimumCalculator(first, second).get_minimum_distance()

        # Assert
        self.assertEqual(expected, actual)

    def test_correct_steps(self):
        # Arrange
        first = ['R8', 'U5', 'L5', 'D3']
        second = ['U7', 'R6', 'D4', 'L4']
        expected = 30

        # Act
        actual = MinimumCalculator(first, second).get_minimum_steps()

        # Assert
        self.assertEqual(expected, actual)
