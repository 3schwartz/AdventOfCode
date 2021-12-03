from year2021.python.day3.day3_func import *

generator = ReportGenerator()

report = [generator.DiagnosticReader(diagnostic.rstrip('\n')) for diagnostic in open('../../data/day3_data.txt')]

finder = RateFinder()

consumption = finder.Consumption(report)

print(f"Part 1: {consumption}")

co2Oxygen = finder.CO2Oxygen(report)

print(f"Part 2: {co2Oxygen}")