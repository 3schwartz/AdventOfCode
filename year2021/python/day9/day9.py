from year2021.python.day9.day9_func import *

inputLines = open('../../data/day9_data.txt').readlines()
analyzer = HeightAnalyzer(inputLines)

risk = analyzer.getRisk()

print(f"Part 1: {risk}")

maxBasinSize = analyzer.getBiggestBasinSize()

print(f"Part 2: {maxBasinSize}")
