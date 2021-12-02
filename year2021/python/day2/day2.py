from year2021.python.day2.day2_func import *

moves = [Move(move) for move in open('../../data/day2_data.txt')]

calculator = RouteCalculator()
position = calculator.GetPosition(moves)

print(f"Part 1: {position}")

aimCalculator = AimCalculator()
aimPosition = aimCalculator.GetPosition(moves)

print(f"Part 2: {aimPosition}")

