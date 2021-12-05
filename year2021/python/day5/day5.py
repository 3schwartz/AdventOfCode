from year2021.python.day5.day5_func import *

lines = [LineFactory.getLine(line.strip()) for line in open('../../data/day5_data.txt')]
finder = IntersectFinder()

pointsOverlap = finder.findStraigtIntersectCount(lines)

print(f"Part 1: {pointsOverlap}")

pointsWithDiagonal = finder.findWithDiagonalIntersect(lines)

print(f"Part 2: {pointsWithDiagonal}")