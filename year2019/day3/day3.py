from day3_func import *


wire_one, wire_two = open('day3_data.txt').read().rstrip('\n').split('\n')
wire_one = wire_one.split(',')
wire_two = wire_two.split(',')

print(f"Part 1: {get_minimum_distance(wire_one, wire_two)}")

print(f"Part 2: {get_minimum_steps(wire_one, wire_two)}")
