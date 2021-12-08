import unittest
from year2021.python.day8.day8_func import *

class TestDay8(unittest.TestCase):

    def test_correct_output(self):
        # Arrange
        oneRow = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        signal = Signal(oneRow)

        # Act
        outputNumber = signal.outputNumber()

        # Assert
        self.assertEqual(5353, outputNumber)


    def test_correct_output(self):
        # Arrange
        oneRow = "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
        signal = Signal(oneRow)

        # Act
        outputNumber = signal.outputNumber()

        # Assert
        self.assertEqual(9781, outputNumber)

    def test_correct_output(self):
        # Arrange
        oneRow = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
        signal = Signal(oneRow)

        # Act
        outputNumber = signal.outputNumber()

        # Assert
        self.assertEqual(8394, outputNumber)

    def test_correct_example_count(self):
        # Arrange
        signals = [Signal(line) for line in open('../../data/day8_data_test.txt')]

        # Act
        outputNumber = sum([signal.outputNumber() for signal in signals])

        # Assert
        self.assertEqual(61229, outputNumber)

    def test_correct_example(self):
        # Arrange
        signals = [Signal(line) for line in open('../../data/day8_data_test.txt')]

        # Act
        uniqueCount = sum([signal.numberUniqueInDigits() for signal in signals])

        # Assert
        self.assertEqual(26, uniqueCount)


    def test_correct_init(self):
        # Arrange
        oneRow = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"

        # Act
        signal = Signal(oneRow)

        # Assert
        self.assertEqual(len(signal.signals), 10)
        self.assertEqual(len(signal.digits), 4)

    def test_correct_number_unique_one_row(self):
        # Arrange
        oneRow = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
        signal = Signal(oneRow)

        # Act
        numberUnique = signal.numberUniqueInDigits()

        # Assert
        self.assertEqual(2, numberUnique)