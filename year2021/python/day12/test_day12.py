import unittest

from year2021.python.day12.day12_func import *


class TestClass(unittest.TestCase):

    def test_correct_distinct_path_3(self):
        # Arrange
        with open('day12_data_test3') as f:
            input_lines = f.read().split('\n')

        path_finder = PathFinder(input_lines)

        # Act
        distinct_paths = path_finder.get_distinct_paths()

        # Assert
        self.assertEqual(226, len(distinct_paths))

    def test_correct_distinct_path_2(self):
        # Arrange
        with open('day12_data_test2') as f:
            input_lines = f.read().split('\n')

        path_finder = PathFinder(input_lines)

        # Act
        distinct_paths = path_finder.get_distinct_paths()

        # Assert
        self.assertEqual(19, len(distinct_paths))

    def test_correct_distinct_path_1(self):
        # Arrange
        with open('day12_data_test.txt') as f:
            input_lines = f.read().split('\n')

        path_finder = PathFinder(input_lines)

        # Act
        distinct_paths = path_finder.get_distinct_paths()

        # Assert
        self.assertEqual(10, len(distinct_paths))

    def test_correct_distinct_path_two_visits_1(self):
        # Arrange
        with open('day12_data_test.txt') as f:
            input_lines = f.read().split('\n')

        path_finder = PathFinder(input_lines)

        # Act
        distinct_paths = path_finder.get_distinct_paths_two_visits()

        # Assert
        self.assertEqual(36, len(distinct_paths))

    def test_correct_distinct_path_two_visits_2(self):
        # Arrange
        with open('day12_data_test2') as f:
            input_lines = f.read().split('\n')

        path_finder = PathFinder(input_lines)

        # Act
        distinct_paths = path_finder.get_distinct_paths_two_visits()

        # Assert
        self.assertEqual(103, len(distinct_paths))

    def test_correct_distinct_path_two_visits_2(self):
        # Arrange
        with open('day12_data_test3') as f:
            input_lines = f.read().split('\n')

        path_finder = PathFinder(input_lines)

        # Act
        distinct_paths = path_finder.get_distinct_paths_two_visits()

        # Assert
        self.assertEqual(3509, len(distinct_paths))
