import common
import math

lines = [(line[0], int(line[1:])) for line in common.get_lines("day12_data.txt")]

position = {'E': 0, 'S': 0, 'W': 0, 'N': 0}
direction = 'E'

for line in lines:
    if line[0] == 'F':
        position[direction] += line[1]

    if line[0] in position.keys():
        position[line[0]] += line[1]

    if line[0] in ['L', 'R']:
        turn = int(line[1] / 90)
        multiplier = 1 if line[0] == 'R' else -1
        direction_idx = (list(position).index(direction) + multiplier * turn) % len(position)
        direction = list(position)[direction_idx]

print(f"Part 1: {abs(position['E'] - position['W']) + abs(position['N'] - position['S'])}")


def rotate(origin: tuple, point: tuple, angle: int):
    ox, oy = origin
    px, py = point
    qx = ox + math.cos(angle) * (px - ox) - math.sin(angle) * (py - oy)
    qy = oy + math.sin(angle) * (px - ox) + math.cos(angle) * (py - oy)
    return int(round(qx)), int(round(qy))


ship = {'x': 0, 'y': 0}
waypoint = {'x': 10, 'y': 1}
directions = {'N': 'y', 'S': 'y', 'W': 'x', 'E': 'x'}

for line in lines:

    if line[0] in directions.keys():
        multiplier = 1 if line[0] in ['N', 'E'] else -1
        waypoint[directions[line[0]]] += multiplier * line[1]

    if line[0] == 'F':
        ship['x'] += waypoint['x'] * line[1]
        ship['y'] += waypoint['y'] * line[1]

    if line[0] in ['L', 'R']:
        multiplier = 1 if line[0] in ['L'] else -1
        waypoint['x'], waypoint['y'] = rotate((0,0), (waypoint['x'], waypoint['y']), math.radians(multiplier*line[1]))

print(f"Part 2: {abs(ship['x']) + abs(ship['y'])}")
