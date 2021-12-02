from abc import ABC, abstractmethod
from typing import List


class Move:
    def __init__(self, moveStr: str):
        self.direction, step = moveStr.split(" ")
        self.step = int(step)


class Validator(ABC):

    @abstractmethod
    def GetPosition(self, moves: List[Move]) ->  int:
        pass

class AimCalculator(Validator):

    def GetPosition(self, moves: List[Move]) -> int:
        position, debt, aim = 0, 0, 0

        for move in moves:
            if move.direction == "forward":
                position += move.step
                debt += aim * move.step
            if move.direction == "down":
                aim += move.step
            if move.direction == "up":
                aim -= move.step

        return position * debt


class RouteCalculator(Validator):

    def GetPosition(self, moves: List[Move]):
        position, debt = 0, 0

        for move in moves:
            if move.direction == "forward":
                position += move.step
            if move.direction == "down":
                debt += move.step
            if move.direction == "up":
                debt -= move.step

        return position * debt
