from year2021.python.day9.day9_func import *

inputLines = open('../../data/day9_data.txt')

heights = HeightCreator.createHeightMap(inputLines)
analyzer = HeightAnalyzer()

risk = analyzer.getRisk(heights)

print(f"Part 1: {risk}")

maxBasinSize = analyzer.getBasinMax(heights)

print(f"Part 2: {maxBasinSize}")
