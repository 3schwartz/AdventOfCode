from year2019.day4.day4_func import *
from itertools import groupby

bounds = (272091,815432)

passed=sum([sequence_pass_rules(s, get_rules()[:2]) for s in range(bounds[0], bounds[1]+1)])

print(f"Part 1: {passed}")

passedSecond=sum([sequence_pass_rules(s, get_rules()) for s in range(bounds[0], bounds[1]+1)])

print(f"Part 2: {passedSecond}")