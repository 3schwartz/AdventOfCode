import unittest
from year2021.python.day3.day3_func import *


class TestDay3(unittest.TestCase):

    def test_correct_oxygen(self):
        # Arrange
        report = [[1, 1, 0], [1, 0, 0], [0, 0, 1], [1, 0, 1]]
        converter = RateFinder()

        # Act
        oxygen = converter.OxygenFinder(report)

        # Assert
        self.assertEqual("101", oxygen)

    def test_correct_binray(self):
        # Arrange
        binary = "01001"
        converter = RateFinder()

        # Act
        numberFromBinary = converter.BinrayToDecimal(binary)

        # Assert
        self.assertEqual(numberFromBinary, 9)

    def test_correct_gamma(self):
        # Arrange
        report = [[1, 0, 0], [1, 0, 0], [1, 0, 1]]
        finder = RateFinder()

        # Act
        epsilon = finder.EpsilonRateFinder(report)

        # Assert
        self.assertEqual(epsilon, "011")

    def test_correct_gamma(self):
        # Arrange
        report = [[1, 0, 0], [1, 0, 0], [1, 0, 1]]
        gammaFinder = RateFinder()

        # Act
        gamma = gammaFinder.GammaRateFinder(report)

        # Assert
        self.assertEqual(gamma, "100")

    def test_correct_number_from_binary(self):
        pass

    def test_split_correct(self):
        # Arrange
        numberString = "112"
        reader = ReportGenerator()

        # Act
        diagnostic = reader.DiagnosticReader(numberString)

        # Assert
        self.assertEqual(len(diagnostic), 3)
        self.assertEqual(diagnostic[0], 1)
        self.assertEqual(diagnostic[1], 1)
        self.assertEqual(diagnostic[2], 2)
