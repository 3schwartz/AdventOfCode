from abc import ABC, abstractmethod
from typing import List
from collections import Counter


class LineFactory:

    @staticmethod
    def getLine(line: str):
        fromPoint, toPoint = LineFactory.getTubleFromStr(line)

        if LineFactory.isVerticalOrHorizontal(fromPoint, toPoint):
            return StraightLine(fromPoint, toPoint)

        if LineFactory.isDiagonal(fromPoint, toPoint):
            return DiagonalLine(fromPoint, toPoint)

        raise Exception("Unknown type of line")

    @staticmethod
    def getTubleFromStr(line: str):
        strFrom, strTo = line.split(" -> ")
        fromPoint = (int(strFrom.split(',')[0]), int(strFrom.split(',')[1]))
        toPoint = (int(strTo.split(',')[0]), int(strTo.split(',')[1]))
        return (fromPoint, toPoint)

    @staticmethod
    def isVerticalOrHorizontal(fromPoint: tuple, toPoint: tuple):
        if fromPoint[0] == toPoint[0]:
            return True
        if fromPoint[1] == toPoint[1]:
            return True
        return False

    @staticmethod
    def isDiagonal(fromPoint: tuple, toPoint: tuple):
        return abs(fromPoint[0] - toPoint[0]) == abs(fromPoint[1] - toPoint[1])

class Line(ABC):

    @abstractmethod
    def getPoints(self):
        pass

    @property
    def fromPoint(self):
        return self._fromPoint

    @fromPoint.setter
    def fromPoint(self, point: tuple):
        self._fromPoint = point

    @property
    def toPoint(self):
        return self._toPoint

    @toPoint.setter
    def toPoint(self, point: tuple):
        self._toPoint = point

class DiagonalLine(Line):

    def __init__(self, fromPoint: tuple, toPoint: tuple):
        self.fromPoint = fromPoint
        self.toPoint = toPoint

    def getPoints(self):
        steps = abs(self.fromPoint[0] - self.toPoint[0])
        xMultiplier = 1 if self.toPoint[0] > self.fromPoint[0] else -1
        yMultiplier = 1 if self.toPoint[1] > self.fromPoint[1] else -1

        x = self.fromPoint[0]
        y = self.fromPoint[1]
        points = [f"{x},{y}"]
        points.append(f"{self.toPoint[0]},{self.toPoint[1]}")

        for i in range(1, steps):
            x += xMultiplier
            y += yMultiplier
            points.append(f"{x},{y}")
        return points

class StraightLine(Line):

    def __init__(self, fromPoint: tuple, toPoint: tuple):
        self.fromPoint = fromPoint
        self.toPoint = toPoint

    def getPoints(self):
        points = []

        if self.fromPoint[0] != self.toPoint[0]:
            multiplier = 1 if self.toPoint[0] > self.fromPoint[0] else -1
            for x in range(multiplier * self.fromPoint[0], multiplier * self.toPoint[0] + 1):
                points.append(f"{multiplier * x},{self.fromPoint[1]}")

        if self.fromPoint[1] != self.toPoint[1]:
            multiplier = 1 if self.toPoint[1] > self.fromPoint[1] else -1
            for y in range(multiplier * self.fromPoint[1], multiplier * self.toPoint[1] + 1):
                points.append(f"{self.fromPoint[0]},{multiplier * y}")

        return points


class IntersectFinder:

    def findWithDiagonalIntersect(self, lines: List[Line]) -> int:
        points = []
        for line in lines:
            points.extend(line.getPoints())

        overlaps = self.getOverlapsAboveThreshold(points)

        return overlaps

    def findStraigtIntersectCount(self, lines: List[Line]) -> int:
        points = []
        for line in lines:
            if type(line) == StraightLine:
                points.extend(line.getPoints())

        overlaps = self.getOverlapsAboveThreshold(points)

        return overlaps

    def getOverlapsAboveThreshold(self, points: List[str], threshold=2):
        overlaps = 0
        intersectDict = Counter(Counter(points).values())
        for i in range(threshold, len(intersectDict) + 1):
            overlaps += intersectDict[i]
        return overlaps
