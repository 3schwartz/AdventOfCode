import unittest
from year2021.python.day10.day10_func import *


class TestDay10(unittest.TestCase):

    def test_correct_score_incomplete(self):
        # Arrange
        inputLines = open('../../data/day10_data_test.txt')
        validator = SyntaxValidator()

        # Act
        score = validator.getIncompleteScore(inputLines)

        # Arrange
        self.assertEqual(288957, score)

    def test_correct_completion(self):
        # Arrange
        incomplete = "<{([{{}}[<[[[<>{}]]]>[]]"
        validator = SyntaxValidator()
        sentence = validator.isCorupted(incomplete)

        # Act
        score = sentence.getIncompleteScore()

        # Arrange
        self.assertEqual(294, score)

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
        sentence = validator.isCorupted(incomplete)

        # Arrange
        self.assertEqual(True, sentence.isCorupted)
        self.assertEqual('}', sentence.coruptedSyntax)

    def test_not_is_corrupted(self):
        # Arrange
        incomplete = "[({(<(())[]>[[{[]{<()<>>"
        validator = SyntaxValidator()

        # Act
        sentence = validator.isCorupted(incomplete)

        # Arrange
        self.assertEqual(False, sentence.isCorupted)
        self.assertEqual("", sentence.coruptedSyntax)
