import common
import math

lines = common.get_lines('day5_data.txt')


def binary_search(from_to: tuple, sequence: list, splitter: str):
    start, end = from_to
    for char in sequence:
        if char == splitter:
            end = math.floor((start + end) / 2)
        else:
            start = math.ceil((start + end) / 2)
    return start


seats = []
for line in lines:
    row = binary_search((0, 127), line[:7], 'F')
    col = binary_search((0, 7), line[7:], 'L')
    seats.append(row * 8 + col)

print(f"Part 1: {max(seats)}")
print(f"Part 2: {[seat for seat in range(min(seats), max(seats)) if seat not in seats]}")