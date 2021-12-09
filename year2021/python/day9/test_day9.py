import unittest
from year2021.python.day9.day9_func import *


class TestDay9(unittest.TestCase):

    def test_correct_basin_max(self):
        # Arrange
        inputLines = open('../../data/day9_data_test.txt')
        heights = HeightCreator.createHeightMap(inputLines)
        analyzer = HeightAnalyzer()


        # Act
        maxBasinSize = analyzer.getBasinMax(heights)

        # Assert
        self.assertEqual(1134, maxBasinSize)

    def test_correct_basin_size_fourth(self):
        # Arrange
        inputLines = open('../../data/day9_data_test.txt')
        heights = HeightCreator.createHeightMap(inputLines)
        analyzer = HeightAnalyzer()
        id = (2, 2)

        # Act
        basinSize = analyzer.getBasinSize(id, heights)

        # Assert
        self.assertEqual(14, basinSize)

    def test_correct_basin_size_third(self):
        # Arrange
        inputLines = open('../../data/day9_data_test.txt')
        heights = HeightCreator.createHeightMap(inputLines)
        analyzer = HeightAnalyzer()
        id = (0, 9)

        # Act
        basinSize = analyzer.getBasinSize(id, heights)

        # Assert
        self.assertEqual(9, basinSize)

    def test_correct_basin_size_second(self):
        # Arrange
        inputLines = open('../../data/day9_data_test.txt')
        heights = HeightCreator.createHeightMap(inputLines)
        analyzer = HeightAnalyzer()
        id = (0,1 )

        # Act
        basinSize = analyzer.getBasinSize(id, heights)

        # Assert
        self.assertEqual(3, basinSize)

    def test_correct_basin_size(self):
        # Arrange
        inputLines = open('../../data/day9_data_test.txt')
        heights = HeightCreator.createHeightMap(inputLines)
        analyzer = HeightAnalyzer()
        id = (4, 6)

        # Act
        basinSize = analyzer.getBasinSize(id, heights)

        # Assert
        self.assertEqual(9, basinSize)

    def test_correct_height(self):
        # Arrange
        inputLines = open('../../data/day9_data_test.txt')
        heights = HeightCreator.createHeightMap(inputLines)
        analyzer = HeightAnalyzer()

        # Act
        risk = analyzer.getRisk(heights)

        # Assert
        self.assertEqual(15, risk)
