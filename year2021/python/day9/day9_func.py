from typing import List
import numpy as np
from functools import reduce


class HeightAnalyzer:

    def __init__(self, inputLines: List[str]):
        heights = [list(line.strip()) for line in inputLines]
        self.heights = np.array(heights, dtype=int)

    def getBiggestBasinSize(self):
        allIsTrue = self.getLowest()
        idx = np.where(allIsTrue == 1)

        basinSizes = [self.getBasinSize(id) for id in zip(idx[0], idx[1])]
        basinSizes.sort(reverse=True)

        basinMaxSize = reduce(lambda a, b: a * b, basinSizes[:3])

        return basinMaxSize

    def getBasinSize(self, id: tuple) -> int:
        sizeBasin = 0
        reamining = [id]
        lookedUpon = 0
        while len(reamining) != lookedUpon:

            current = reamining[lookedUpon]

            lowerNeighbors = []

            for i in (-1, 0, 1):
                for j in (-1, 0, 1):
                    if abs(i + j) == 2 or i + j == 0:
                        continue
                    idxRow = current[0] + i
                    idxColumn = current[1] + j

                    if idxRow < 0 or idxRow >= self.heights.shape[0]:
                        continue
                    if idxColumn < 0 or idxColumn >= self.heights.shape[1]:
                        continue
                    if self.heights[current[0], current[1]] < self.heights[idxRow, idxColumn]:
                        lowerNeighbors.append((idxRow, idxColumn))

            if len(lowerNeighbors) > 0:
                sizeBasin += 1
            for lowerNeighbor in lowerNeighbors:
                if lowerNeighbor not in reamining:
                    reamining.append(lowerNeighbor)
            lookedUpon += 1

        return sizeBasin

    def getRisk(self) -> int:
        allIsTrue = self.getLowest()
        return int((self.heights * allIsTrue + allIsTrue).sum())

    def getLowest(self) -> np.array:
        below = self.belowRule()
        above = self.aboveRule()
        right = self.rightRule()
        left = self.leftRule()
        allIsTrue = np.floor((above.astype(int) + below + right + left) / 4)
        return allIsTrue

    def belowRule(self) -> np.array:
        below = self.getAllTrue()
        below[:-1, :] = self.heights[:-1, :] < self.heights[1:, :]
        return below

    def aboveRule(self) -> np.array:
        above = self.getAllTrue()
        above[1:, :] = self.heights[:-1, :] > self.heights[1:, :]
        return above

    def rightRule(self) -> np.array:
        right = self.getAllTrue()
        right[:, :-1] = self.heights[:, :-1] < self.heights[:, 1:]
        return right

    def leftRule(self) -> np.array:
        left = self.getAllTrue()
        left[:, 1:] = self.heights[:, :-1] > self.heights[:, 1:]
        return left

    def getAllTrue(self):
        return np.ones(shape=self.heights.shape, dtype=bool)
