def create_square(lines):
    return {(i, j): int(lines[i][j]) for i in range(len(lines)) for j in range(len(lines[0]))}


class Square:
    def __init__(self, lines):
        self.square = create_square(lines)
        self.flash_count = 0
        self.flashed_invoked = 0
        self.all_flash = None

    def flash(self):
        self.flashed_invoked += 1

        flashed = set()
        for tile in self.square:
            self.square[tile] += 1
            if self.square[tile] > 9 and tile not in flashed:
                flashed.add(tile)
                neighbors = [neighbor for neighbor in self.get_neighbors(tile) if not self.square.get(neighbor) is None]
                self.flash_neighbors(neighbors, flashed)

        flash_in_current_flash = 0
        for tile in self.square:
            if self.square[tile] > 9:
                flash_in_current_flash += 1
                self.flash_count += 1
                self.square[tile] = 0

        if flash_in_current_flash == len(self.square):
            self.all_flash = self.flashed_invoked

    def get_neighbors(self, tile):
        return [(tile[0] + i, tile[1] + j)
                for i in range(-1, 2)
                for j in range(-1, 2)
                if not (i == 0 and j == 0)]

    def flash_neighbors(self, neighbors, flashed: set):
        for nb in neighbors:
            self.square[nb] += 1
            if self.square[nb] > 9 and nb not in flashed:
                flashed.add(nb)
                neighbors = [neighbor for neighbor in self.get_neighbors(nb) if not self.square.get(neighbor) is None]
                self.flash_neighbors(neighbors, flashed)

    def __getitem__(self, item):
        return self.square[item]

    def __eq__(self, other):
        for tile in self.square:
            if self.square[tile] != other[tile]:
                print(f"Error in tile {tile} with value {self.square[tile]} and {other[tile]}")
                return False
        return True
