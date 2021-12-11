import unittest
from year2021.python.day10.day10_func import *


class TestDay10(unittest.TestCase):

    def test_correct_score(self):
        # Arrange
        inputLines = open('../../data/day10_data_test.txt')
        validator = SyntaxValidator()

        # Act
        score = validator.getScore(inputLines)

        # Arrange
        self.assertEqual(26397, score)

    def test_is_corrupted(self):
        # Arrange
        incomplete = "{([(<{}[<>[]}>{[]{[(<()>"
        validator = SyntaxValidator()

        # Act
        isIncomplete = validator.isCorupted(incomplete)

        # Arrange
        self.assertEqual(True, isIncomplete[0])
        self.assertEqual('}', isIncomplete[1])

    def test_not_is_corrupted(self):
        # Arrange
        incomplete = "[({(<(())[]>[[{[]{<()<>>"
        validator = SyntaxValidator()

        # Act
        isIncomplete = validator.isCorupted(incomplete)

        # Arrange
        self.assertEqual(False, isIncomplete[0])
        self.assertEqual("", isIncomplete[1])
