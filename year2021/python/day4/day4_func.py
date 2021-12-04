from typing import List
import numpy as np


class BingoGame:
    def __init__(self):
        self.sequence = []
        self.boards = []

    def setUpGame(self, input: List[str]) -> None:
        self.sequence = [int(number) for number in input[0].split(',')]
        self.boards = []

        for board in input[1:]:
            self.boards.append(Board(board))

    def winGame(self) -> int:
        for number in self.sequence:
            for board in self.boards:
                boardBingo = board.notify(number)

                if boardBingo:
                    return board.getScore(number)

    def loseGame(self) -> int:
        winners = set()
        for number in self.sequence:
            for idx in range(len(self.boards)):
                if idx in winners:
                    continue
                board = self.boards[idx]
                boardBingo = board.notify(number)

                if boardBingo:
                    winners.add(idx)
                    if len(winners) == len(self.boards):
                        return board.getScore(number)


class Board:

    def __init__(self, inputBoard: str):
        board = []
        for line in inputBoard.lstrip().replace('  ', ' ').replace('\n ', '\n').split('\n'):
            board.append([int(number) for number in line.split(' ')])
        self.board = np.array(board)
        self.notified = np.zeros(self.board.shape, dtype=bool)

    def notify(self, number: int) -> bool:
        idx = np.where(self.board == number)
        for id in zip(idx[0], idx[1]):
            self.notified[id] = 1

        return self.validateBingo()

    def validateBingo(self):
        for column in self.notified.transpose():
            if sum(column) == self.notified.shape[1]:
                return True
        for row in self.notified:
            if sum(row) == self.notified.shape[0]:
                return True
        return False

    def getScore(self, lastNumber: int):
        notNotified = self.board[~self.notified]
        return sum(notNotified) * lastNumber
