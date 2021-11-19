from day3_func import *


wire_one, wire_two = open('../../data/day3_data.txt').read().rstrip('\n').split('\n')
wire_one = wire_one.split(',')
wire_two = wire_two.split(',')

calculator = MinimumCalculator(wire_one, wire_two)

print(f"Part 1: {calculator.get_minimum_distance()}")

print(f"Part 2: {calculator.get_minimum_steps()}")
