import unittest
from year2021.python.day7.day7_func import *

class TestDay7(unittest.TestCase):

    def test_correct_fuel_factorial(self):
        # Arrange
        input = "16,1,2,0,4,2,7,1,2,14"
        minimizer = BinomialMinimizer()

        # Act
        minFuel = minimizer.getLowestFuel(input)

        # Arrange
        self.assertEqual(168, minFuel)

    def test_correct_fuel(self):
        # Arrange
        input = "16,1,2,0,4,2,7,1,2,14"
        minimizer = StepMinimizer()

        # Act
        minFuel = minimizer.getLowestFuel(input)

        # Arrange
        self.assertEqual(37, minFuel)