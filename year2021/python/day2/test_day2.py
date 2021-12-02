import unittest
from year2021.python.day2.day2_func import *


class TestDay2(unittest.TestCase):

    def test_correct_aim(self):
        # Arrange
        moves = [Move("forward 5"),
                 Move("down 5"),
                 Move("forward 8"),
                 Move("up 3")]

        calculator = AimCalculator()

        # Act
        position = calculator.GetPosition(moves)

        # Assert
        self.assertEqual(position, 520)

    def test_correct_position(self):
        # Arrange
        moves = [Move("forward 5"),
                 Move("down 5"),
                 Move("forward 8"),
                 Move("up 3")]

        routeCalculator = RouteCalculator()

        # Act
        position = routeCalculator.GetPosition(moves)

        # Assert
        self.assertEqual(position, 26)

    def test_create_movement(self):
        # Arrange
        inputString = "forward 5"

        # Act
        move = Move(inputString)

        # Assert
        self.assertEqual(move.direction, "forward")
        self.assertEqual(move.step, 5)
