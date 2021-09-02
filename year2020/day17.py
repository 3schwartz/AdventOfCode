import common


def get_neighbors3d(cube):
    return [(cube[0] + i, cube[1] + j, cube[2] + k)
            for i in range(-1, 2)
            for j in range(-1, 2)
            for k in range(-1, 2)
            if not (i == 0 and j == 0 and k == 0)]


def get_active_neighbors(cubes, neighbors):
    return sum([1 for neighbor in neighbors if cubes.get(neighbor) == '#'])


def do_cycles(cubes, get_neighbors, cycles):
    for i in range(cycles):
        cubes_new = {}

        for cube in cubes:

            neighbors = get_neighbors(cube)

            active_neighbors = get_active_neighbors(cubes, neighbors)

            if cubes[cube] == '#':
                if active_neighbors in [2, 3]:
                    cubes_new[cube] = '#'
                else:
                    cubes_new[cube] = '.'

                for neighbor in neighbors:
                    if neighbor not in cubes:
                        active_neighbor_neighbors = get_active_neighbors(cubes, get_neighbors(neighbor))
                        if active_neighbor_neighbors == 3:
                            cubes_new[neighbor] = '#'
            else:
                if active_neighbors == 3:
                    cubes_new[cube] = '#'
                else:
                    cubes_new[cube] = '.'

        cubes = cubes_new

    return cubes


lines = common.get_lines("day17_data.txt")

cubes = {(i, j, 0): lines[i][j] for i in range(len(lines)) for j in range(len(lines[0]))}

cubes3d = do_cycles(cubes, get_neighbors3d, 6)

print(f"Part 1: {list(cubes3d.values()).count('#')}")


def get_neighbors4d(cube):
    return [(cube[0] + i, cube[1] + j, cube[2] + k, cube[3] + w)
            for i in range(-1, 2)
            for j in range(-1, 2)
            for k in range(-1, 2)
            for w in range(-1, 2)
            if not (i == 0 and j == 0 and k == 0 and w == 0)]


cubes = {(i, j, 0, 0): lines[i][j] for i in range(len(lines)) for j in range(len(lines[0]))}

cubes4d = do_cycles(cubes, get_neighbors4d, 6)

print(f"Part 2: {list(cubes4d.values()).count('#')}")
