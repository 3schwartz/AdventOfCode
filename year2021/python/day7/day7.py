from year2021.python.day7.day7_func import *

crabsHorizontalString = open('../../data/day7_data.txt').readline()

minimizer = StepMinimizer()

minFuel = minimizer.getLowestFuel(crabsHorizontalString)

print(f"Part 1: {minFuel}")

minimizerBinomial = BinomialMinimizer()

minBinomial = minimizerBinomial.getLowestFuel(crabsHorizontalString)

print(f"Part 2: {minBinomial}")
