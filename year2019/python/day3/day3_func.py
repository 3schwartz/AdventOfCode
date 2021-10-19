def get_minimum_distance(wire_one: list, wire_two: list) -> int:
    places_wire_one = set(get_coordinates(wire_one).keys())
    places_wire_two = set(get_coordinates(wire_two).keys())

    coordinates_intersection = places_wire_one.intersection(places_wire_two)

    distances = [abs(coordinate[0]) + abs(coordinate[1]) for coordinate in list(coordinates_intersection) if
                 coordinate[0] != 0 or coordinate[1] != 0]
    return min(distances)


def get_minimum_steps(wire_one: list, wire_two: list) -> int:
    places_wire_one = get_coordinates(wire_one)
    places_wire_two = get_coordinates(wire_two)

    coordinates_intersection = set(places_wire_one.keys()).intersection(set(places_wire_two.keys()))

    steps = [places_wire_one[intersection] + places_wire_two[intersection] for intersection in coordinates_intersection
             if intersection != (0, 0)]

    return min(steps)


def add_range(coordinates: dict, coord: str, value: int, positive: bool, places: dict):
    multiplier = 1 if positive else -1
    before = coordinates[coord]
    after = before + multiplier * value
    between = range(before + multiplier, after + multiplier, multiplier)

    steps = coordinates['s'] + 1

    for i in between:
        if coord == 'x':
            places[(i, coordinates['y'])] = steps
        else:
            places[(coordinates['x'], i)] = steps
        steps += 1

    if coord == 'x':
        coordinates['x'] += multiplier * value
    else:
        coordinates['y'] += multiplier * value
    coordinates['s'] += value


def get_coordinates(wire: list) -> dict:
    coordinates = {'x': 0, 'y': 0, 's': 0}
    places = {}

    for direction in wire:
        key, value = direction[0], int(direction[1:])

        if key == 'U':
            add_range(coordinates, 'x', value, True, places)

        if key == 'D':
            add_range(coordinates, 'x', value, False, places)

        if key == 'R':
            add_range(coordinates, 'y', value, True, places)

        if key == 'L':
            add_range(coordinates, 'y', value, False, places)

    return places
