from typing import List
from collections import Counter


class Line:

    def __init__(self, line: str):
        strFrom, strTo = line.split(" -> ")
        self.fromPoint = (int(strFrom.split(',')[0]), int(strFrom.split(',')[1]))
        self.toPoint = (int(strTo.split(',')[0]), int(strTo.split(',')[1]))

    def isVerticalOrHorizontal(self):
        if self.fromPoint[0] == self.toPoint[0]:
            return True
        if self.fromPoint[1] == self.toPoint[1]:
            return True
        return False

    def isDiagonal(self):
        return abs(self.fromPoint[0] - self.toPoint[0]) == abs(self.fromPoint[1] - self.toPoint[1])

    def getDiagonal(self):
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

    def getStraitPoint(self):
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
            if line.isVerticalOrHorizontal():
                points.extend(line.getStraitPoint())
            if line.isDiagonal():
                points.extend(line.getDiagonal())

        overlaps = self.getOverlapsAboveThreshold(points)

        return overlaps

    def findStraigtIntersectCount(self, lines: List[Line]) -> int:
        points = []
        for line in lines:
            if not line.isVerticalOrHorizontal():
                continue
            points.extend(line.getStraitPoint())

        overlaps = self.getOverlapsAboveThreshold(points)

        return overlaps

    def getOverlapsAboveThreshold(self, points: List[str], threshold=2):
        overlaps = 0
        intersectDict = Counter(Counter(points).values())
        for i in range(threshold, len(intersectDict) + 1):
            overlaps += intersectDict[i]
        return overlaps
