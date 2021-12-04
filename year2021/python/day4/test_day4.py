import unittest
from year2021.python.day4.day4_func import *


class TestDay4(unittest.TestCase):

    def test_board_score(self):
        # Arrange
        inputBoard = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19"
        board = Board(inputBoard)
        board.notified[:, 0] = 1

        # Act
        score = board.getScore(2)

        # Assert
        self.assertEqual(484, score)

    def test_board_when_bingo(self):
        # Arrange
        inputBoard = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19"
        board = Board(inputBoard)

        # Act
        board.notify(22)
        board.notify(13)
        board.notify(17)
        noBingo = board.notify(11)
        bingo = board.notify(0)

        # Assert
        self.assertEqual(False, noBingo)
        self.assertEqual(True, bingo)

    def test_when_notify_then_one_set(self):
        # Arrange
        inputBoard = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19"
        board = Board(inputBoard)

        # Act
        bingo = board.notify(22)

        # Assert
        self.assertEqual(False, bingo)
        self.assertEqual(True, board.notified[0, 0])

    def test_correct_bingo(self):
        # Arrange
        input = [stack for stack in open('./day4_data_test.txt').read().split('\n\n')]
        game = BingoGame()

        # Act
        game.setUpGame(input)

        # Arrange
        self.assertEqual(game.sequence[0], 7)
        self.assertEqual(len(game.boards), 3)

    def test_correct_bingo(self):
        # Arrange
        input = [stack for stack in open('./day4_data_test.txt').read().split('\n\n')]
        game = BingoGame()
        game.setUpGame(input)

        # Act
        score = game.winGame()

        # Arrange
        self.assertEqual(4512, score)

    def test_losing_bingo(self):
        # Arrange
        input = [stack for stack in open('./day4_data_test.txt').read().split('\n\n')]
        game = BingoGame()
        game.setUpGame(input)

        # Act
        score = game.loseGame()

        # Arrange
        self.assertEqual(1924, score)

    def test_correct_board_init(self):
        # Arrange
        inputBoard = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19"

        # Act
        board = Board(inputBoard)

        # Assert
        self.assertEqual((5, 5), board.board.shape)
        self.assertEqual((5, 5), board.notified.shape)
