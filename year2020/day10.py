import common
from collections import defaultdict

lines = [int(line) for line in common.get_lines("day10_data.txt")]
lines.append(max(lines) + 3)
lines.sort()

last = 0
jolts = defaultdict(int)

for line in lines:
    jolts[line - last] += 1
    last = line

print(f"Part 1: {jolts[1] * jolts[3]}")

combinations = defaultdict(int)
combinations[0] = 1
for jolt in lines:
    combinations[jolt] = combinations[jolt-3] + combinations[jolt-2] + combinations[jolt-1]

print(f"Part 2: {combinations[lines[-1]]}")
