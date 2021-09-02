import common

lines = [list(line) for line in common.get_lines("day11_data.txt")]

rows, cols = len(lines), len(lines[0])

deltas = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)]


def get_occupied(r: int, c: int, grid: list):
    count = 0
    for i, j in deltas:
        xr, xc = r + i, c + j
        if 0 <= xr < rows and 0 <= xc < cols and grid[xr][xc] == '#':
            count += 1
    return count


def get_occupied_long(r: int, c: int, grid: list):
    count = 0
    for i, j in deltas:
        xr, xc = r + i, c + j
        while 0 <= xr < rows and 0 <= xc < cols:
            if grid[xr][xc] == '#':
                count += 1
                break
            if grid[xr][xc] == 'L':
                break
            xr += i
            xc += j
    return count


def get_equilibrium(threshold: int, occupied):
    grid_out = [r.copy() for r in lines]
    while True:
        grid_inner = [r.copy() for r in grid_out]
        for i, r in enumerate(grid_inner):
            for j, c in enumerate(r):
                count = occupied(i, j, grid_inner)
                if c == 'L' and count == 0:
                    grid_out[i][j] = '#'
                if c == '#' and count >= threshold:
                    grid_out[i][j] = 'L'
        if all(grid_out[i][j] == grid_inner[i][j] for i in range(rows) for j in range(cols)):
            break
    return sum(1 for i in range(rows) for j in range(cols) if grid_out[i][j] == '#')


print(f"Part 1: {get_equilibrium(4, get_occupied)}")
print(f"Part 1: {get_equilibrium(5, get_occupied_long)}")
