from year2021.python.day11.day11_func import *

with open('../../data/day11_data.txt') as f:
    inputLines = f.read().split('\n\n')

lines = inputLines[0].split('\n')
square = Square(lines)


while square.all_flash is None:
    square.flash()

    if square.flashed_invoked == 100:
        print(f"Part 1: {square.flash_count}")


print(f"Part 2: {square.flashed_invoked}")