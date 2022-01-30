from functools import reduce, partial
from typing import List, Dict


class HeightCreator:

    @staticmethod
    def createHeightMap(inputMatrix: List[str]):
        heights = {(i, j): int(height) for i, l in enumerate(inputMatrix)
                   for j, height in enumerate(l.strip())}
        return heights


class HeightAnalyzer:

    def getNeighbours(self, row: int, col: int, heights: Dict[tuple, int]) -> List[tuple]:
        neighboursPositions = [(row, col - 1), (row, col + 1), (row - 1, col), (row + 1, col)]
        neighbours = [position for position in neighboursPositions if position in heights]
        return neighbours

    def isLow(self, position: tuple, heights: Dict[tuple, int]) -> bool:
        lowerNeighbours = [heights[position] < heights[neighbour]
                           for neighbour in self.getNeighbours(*position, heights)]
        return all(lowerNeighbours)

    def getLowest(self, heights: Dict[tuple, int]):
        lowPosition = [position for position in heights if self.isLow(position, heights)]
        return lowPosition

    def getRisk(self, heights: Dict[tuple, int]) -> int:
        lowPositions = self.getLowest(heights)
        return sum(heights[position] + 1 for position in lowPositions)

    def getBasinSize(self, position: tuple, heights: Dict[tuple, int]) -> int:
        if position not in heights: return 0
        if heights[position] == 9: return 0
        del heights[position]
        nextBasin = partial(self.getBasinSize, heights=heights)
        basinCount = 1 + sum(map(nextBasin,
                                 self.getNeighbours(*position, heights)))
        return basinCount

    def getBasinMax(self, heights: Dict[tuple, int]) -> int:
        lowPositions = self.getLowest(heights)
        basinSizes = [self.getBasinSize(position, heights.copy())
                      for position in lowPositions]
        basinMaxSize = reduce(lambda a, b: a * b,
                              sorted(basinSizes, reverse=True)[:3])
        return basinMaxSize

