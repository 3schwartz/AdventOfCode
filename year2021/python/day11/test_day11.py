import unittest
from year2021.python.day11.day11_func import *


def get_neighbors(tile):
    return [(tile[0] + i, tile[1] + j)
            for i in range(-1, 2)
            for j in range(-1, 2)
            if not (i == 0 and j == 0)]


def flash(tiles):
    flashed = set()

    for tile in tiles:
        tiles[tile] += 1
        if tiles[tile] > 9 and tile not in flashed:
            flashed.add(tile)
            neighbors = [neighbor for neighbor in get_neighbors(tile) if not tiles.get(neighbor) is None]
            flash_neighbors(tiles, neighbors, flashed)

    for tile in tiles:
        if tiles[tile] > 9:
            tiles[tile] = 0


def flash_neighbors(tiles, neighbors, flashed: set):
    for nb in neighbors:
        tiles[nb] += 1
        if tiles[nb] > 9 and nb not in flashed:
            flashed.add(nb)
            neighbors = [neighbor for neighbor in get_neighbors(nb) if not tiles.get(neighbor) is None]
            flash_neighbors(tiles, neighbors, flashed)


class TestClass(unittest.TestCase):

    def test_correct_after_one_cycle(self):
        with open('day11_data_test.txt') as f:
            inputLines = f.read().split('\n\n')

        lines = inputLines[0].split('\n')
        lines_first = inputLines[1].split('\n')
        lines_second = inputLines[2].split('\n')

        square = create_square(lines)
        square_first_flash = create_square(lines_first)
        square_second_flash = create_square(lines_second)

        flash(square)

        for tile in square:
            if square[tile] != square_first_flash[tile]:
                print(f"Error in tile {tile} with value {square[tile]} and {square_second_flash[tile]}")

        flash(square)

        for tile in square:
            if square[tile] != square_second_flash[tile]:
                print(f"Error in tile {tile} with value {square[tile]} and {square_second_flash[tile]}")

        square_class = Square(lines)
        square_class_first = Square(lines_first)
        square_class_second = Square(lines_second)

        square_class.flash()

        self.assertEqual(square_class, square_class_first)
        self.assertEqual(square_class.flash_count, 9)

        square_class.flash()

        self.assertEqual(square_class, square_class_second)
        self.assertEqual(square_class.flash_count, 9)
