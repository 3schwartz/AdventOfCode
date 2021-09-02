import common
from functools import reduce

lines = common.get_lines("day3_data.txt")


def tree_count(lines: list, slope: tuple):
    trees_count, column = 0, 0

    for row, line in enumerate(lines):
        if row % slope[1] == 0:
            if line[column] == "#":
                trees_count += 1
            column = (slope[0] + column) % len(line)
    return trees_count


print(f"Part 1: {tree_count(lines, (3, 1))}")

slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

print(f"Part 2: {reduce(lambda x, y: x * y, [tree_count(lines, slope) for slope in slopes])}")
