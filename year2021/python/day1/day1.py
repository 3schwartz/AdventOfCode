from year2021.python.day1.day1_func import *

debts = [int(debt) for debt in open('../../data/day1_data.txt')]

sonarSingle = SonarSingle()
singleDebt = sonarSingle.GetDebtCount(debts)

print(f"Part 1: {singleDebt}")

sonarWindow = SonarWindow()
windowDebt = sonarWindow.GetDebtCount(debts)

print(f"Part 2: {windowDebt}")
