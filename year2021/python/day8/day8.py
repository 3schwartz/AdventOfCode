from year2021.python.day8.day8_func import *

signals = [Signal(line) for line in open('../../data/day8_data.txt')]

uniqueCount = sum([signal.numberUniqueInDigits() for signal in signals])

print(f"Part 1: {uniqueCount}")

outputNumber = sum([signal.outputNumber() for signal in signals])

print(f"Part 2: {outputNumber}")