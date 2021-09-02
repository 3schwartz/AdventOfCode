import common
import re
from collections import defaultdict

lines = common.get_lines('day24_data.txt')
floor = defaultdict(lambda: False)

for line in lines:
    directions = re.findall(r"e|se|sw|w|nw|ne", line)
    ns = directions.count('sw') + directions.count('se') - directions.count('nw') - directions.count('ne')
    we = directions.count('ne') + directions.count('e') - directions.count('sw') - directions.count('w')

    floor[(ns, we)] = not floor[(ns, we)]

print(f"Part 1: {sum(floor.values())}")

neighbors = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)]

for _ in range(100):
    turned = defaultdict(lambda: False)

    for tile in floor.keys():
        for neighbor in neighbors:
            neighbor_coordinates = (tile[0] + neighbor[0], tile[1] + neighbor[1])
            if neighbor_coordinates not in turned:
                turned[neighbor_coordinates] = floor.get(neighbor_coordinates, False)

    for tile, color in turned.items():
        blacks = sum([floor[(tile[0] + neighbor[0], tile[1] + neighbor[1])] for neighbor in neighbors])

        if color:
            if not blacks or blacks > 2:
                turned[tile] = False
        else:
            if blacks == 2:
                turned[tile] = True

    floor = turned

print(f"Part 2: {sum(floor.values())}")
