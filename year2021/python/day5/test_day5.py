import unittest
from year2021.python.day5.day5_func import *

class TestDay5(unittest.TestCase):

    def test_is_diagonal_false(self):
        # Arrange
        lineStr = "9,7 -> 7,8"
        line = Line(lineStr)

        # Act
        isDiagonal = line.isDiagonal()

        # Assert
        self.assertEqual(False, isDiagonal)


    def test_is_diagonal_true(self):
        # Arrange
        lineStr = "9,7 -> 7,9"
        line = Line(lineStr)

        # Act
        isDiagonal = line.isDiagonal()

        # Assert
        self.assertEqual(True, isDiagonal)

    def test_get_correct_diagonal_second(self):
        # Arrange
        lineStr = "1,1 -> 3,3"
        line = Line(lineStr)

        # Act
        points = line.getDiagonal()

        # Assert
        self.assertEqual(set(["1,1", "2,2", "3,3"]), set(points))

    def test_get_correct_diagonal_third(self):
        # Arrange
        lineStr = "8,0 -> 0,8"
        line = Line(lineStr)

        # Act
        points = line.getDiagonal()

        # Assert
        self.assertEqual(set(["8,0", "7,1", "6,2", "5,3", "4,4", "3,5", "2,6", "1,7", "0,8"]), set(points))

    def test_get_correct_diagonal(self):
        # Arrange
        lineStr = "9,7 -> 7,9"
        line = Line(lineStr)

        # Act
        points = line.getDiagonal()

        # Assert
        self.assertEqual(set(["9,7", "8,8", "7,9"]), set(points))

    def test_correct_overlap_diagonal(self):
        # Arrange
        lines = [Line(line.strip()) for line in open('../../data/day5_data_test.txt')]
        finder = IntersectFinder()

        # Act
        pointsOverlap = finder.findWithDiagonalIntersect(lines)

        # Assert
        self.assertEqual(12, pointsOverlap)

    def test_correct_overlap(self):
        # Arrange
        lines = [Line(line.strip()) for line in open('../../data/day5_data_test.txt')]
        finder = IntersectFinder()

        # Act
        pointsOverlap = finder.findStraigtIntersectCount(lines)

        # Assert
        self.assertEqual(5, pointsOverlap)

    def test_get_strait_points_y(self):
        # Arrange
        lineStr = "9,7 -> 7,7"
        line = Line(lineStr)

        # Act
        points = line.getStraitPoint()

        # Assert
        self.assertEqual(set(["9,7", "8,7", "7,7"]), set(points))

    def test_get_strait_points_x(self):
        # Arrange
        lineStr = "1,1 -> 1,3"
        line = Line(lineStr)

        # Act
        points = line.getStraitPoint()

        # Assert
        self.assertEqual(["1,1", "1,2", "1,3"], points)

    def test_line_is_horizontal_or_vertical(self):
        # Arrange
        lineTrue = "0,9 -> 5,9"
        lineFalse = "0,1 -> 5,9"

        # Act
        lineCorrect = Line(lineTrue)
        lineNotCorrect = Line(lineFalse)

        # Assert
        self.assertEqual(True, lineCorrect.isVerticalOrHorizontal())
        self.assertEqual(False, lineNotCorrect.isVerticalOrHorizontal())

    def test_correct_line_init(self):
        # Arrange
        lineStr = "0,9 -> 5,9"

        # Act
        line = Line(lineStr)

        # Assert
        self.assertEqual((0,9), line.fromPoint)
        self.assertEqual((5,9), line.toPoint)