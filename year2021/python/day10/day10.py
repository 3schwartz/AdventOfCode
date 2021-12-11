from year2021.python.day10.day10_func import *

inputLines = open('../../data/day10_data.txt').readlines()
validator = SyntaxValidator()

score = validator.getScore(inputLines)

print(f"Part 1: {score}")

inCompleteScore = validator.getIncompleteScore(inputLines)

print(f"Part 2: {inCompleteScore}")