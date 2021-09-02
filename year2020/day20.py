import os
import sys
import math


def transposed(tile):
    return [''.join(row) for row in zip(*tile)]


def reversed_tile(tile):
    return [''.join(reversed(row)) for row in tile]


def rotations(tile):
    tiles = [tile]
    for _ in range(3):
        tiles.append(reversed_tile(transposed(tiles[-1])))
    return tiles


def all_possible_forms(tile):
    return rotations(tile) + rotations(transposed(tile))


tiles = {}

with open(os.path.join(sys.path[0], 'day20_data.txt')) as file:
    for tile in file.read().split('\n\n'):
        lines = tile.split('\n')
        idx = int(lines[0].split(' ')[1][:-1])
        tiles[idx] = all_possible_forms(lines[1:])

one_dimension = int(math.sqrt(len(tiles)))
arranged = [[0] * one_dimension for _ in range(one_dimension)]

stack = list(reversed(list((r, c) for c in range(one_dimension) for r in range(one_dimension))))


def solve():
    if not stack:
        print(arranged[0][0][0] * arranged[-1][0][0] * arranged[0][-1][0] * arranged[-1][-1][0])
        return True
    (r, c) = stack.pop()
    for idx in list(tiles):
        tile_group = tiles[idx]
        del tiles[idx]
        for tile in tile_group:
            if r > 0:
                if arranged[r - 1][c][1][-1] != tile[0]:
                    continue
            if c > 0:
                if list(row[-1] for row in arranged[r][c - 1][1]) != list(row[0] for row in tile):
                    continue
            arranged[r][c] = (idx, tile)
            if solve():
                return True
        tiles[idx] = tile_group
    stack.append((r, c))


print(f"Part 1: {solve()}")


def remove_border(tile):
    return [row[1:-1] for row in tile[1:-1]]


def get_cell_in_arranged(r, c):
    return trimmed_arranged[r // tile_dimension][c // tile_dimension][r % tile_dimension][c % tile_dimension]


trimmed_arranged = [[remove_border(tile[1]) for tile in row] for row in arranged]
tile_dimension = len(trimmed_arranged[0][0])

arranged_joined = [''.join(get_cell_in_arranged(r, c) for c in range(tile_dimension * one_dimension)) for r in
                   range(tile_dimension * one_dimension)]

for pattern in all_possible_forms(['                  # ', '#    ##    ##    ###', ' #  #  #  #  #  #   ']):
    matches = 0
    for dr in range(len(arranged_joined) - len(pattern) + 1):
        for dc in range(len(arranged_joined[0]) - len(pattern[0]) + 1):
            matches += all(pattern[r][c] == ' ' or arranged_joined[r + dr][c + dc] == '#'
                           for r in range(len(pattern))
                           for c in range(len(pattern[0])))
    if matches:
        print(''.join(arranged_joined).count('#') - ''.join(pattern).count('#') * matches)
        break
