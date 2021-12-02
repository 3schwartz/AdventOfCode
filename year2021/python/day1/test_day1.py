import unittest
from year2021.python.day1.day1_func import *


class TestDay1(unittest.TestCase):

    def test_increase_window(self):
        # Arrange
        debts = [1, 2, 3, 0, 9, 10]
        sonar = SonarWindow()

        # Act
        debtCount = sonar.GetDebtCount(debts)

        # Arrange
        self.assertEqual(debtCount, 2)

    def test_increase_seq(self):
        # Arrange
        debts = [1, 2, 3, 3, 4]
        sonar = SonarSingle()

        # Act
        debtCount = sonar.GetDebtCount(debts)

        # Arrange
        self.assertEqual(debtCount, 3)
