import unittest

from year2021.python.day6.day6_func import *

class TestDay6(unittest.TestCase):

    def test_correct_spawn_80(self):
        # Arrange
        fishes = [0, 1, 1, 2, 1, 0, 0, 0, 0]
        spawn = FishSpawn()

        # Act
        numberOfFishes80Days = spawn.spawn(fishes, 80)

        # Assert
        self.assertEqual(5934, numberOfFishes80Days)

    def test_correct_spawn_18(self):
        # Arrange
        fishes = [0, 1, 1, 2, 1, 0, 0, 0, 0]
        spawn = FishSpawn()

        # Act
        numberOfFishes18Days = spawn.spawn(fishes, 18)

        # Assert
        self.assertEqual(26, numberOfFishes18Days)

    def test_correct_init(self):
        # Arrange
        inputString = "3,4,3,1,2"

        # Act
        fishes = FishCreator.initFishes(inputString)

        # Arrange
        self.assertEqual([0, 1, 1, 2, 1, 0, 0, 0, 0], fishes)

    def test_correct_lanternfish_count_same(self):
        # Arrange
        fishes = [0, 1, 1, 2, 1, 0, 0, 0, 0]
        spawn = FishSpawn()

        # Act
        numberOfFishes18Days = spawn.spawn(fishes, 18)
        numberOfFishes80Days = spawn.spawn(fishes, 80)

        # Assert
        self.assertEqual(26, numberOfFishes18Days)
        self.assertEqual(5934, numberOfFishes80Days)
