from year2021.python.day12.day12_func import *

with open('../../data/day12_data.txt') as f:
    input_lines = f.read().split('\n')

path_finder = PathFinder(input_lines)

distinct_paths = path_finder.get_distinct_paths()

print(f"Part 1: {len(distinct_paths)}")

distinct_paths_two_visits = path_finder.get_distinct_paths_two_visits()

print(f"Part 2: {len(distinct_paths_two_visits)}")
