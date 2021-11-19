class Move:
    def __init__(self, direction: str):
        self.direction = direction[:1]
        self.steps = int(direction[1:])


class Wire:
    def __init__(self, moves: list):
        self.moves = []
        for wire in moves:
            self.moves.append(Move(wire))


class MinimumCalculator:

    def __init__(self, move_first, move_second):
        coord_cal = CoordinateCalculator()
        wire_one = Wire(move_first)
        wire_second = Wire(move_second)

        self.places_first = coord_cal.calculate_coordinates(wire_one)
        self.places_second = coord_cal.calculate_coordinates(wire_second)

    def get_minimum_distance(self):
        intersections = self.get_intersection()

        distances = [abs(coordinate[0]) + abs(coordinate[1]) for coordinate in list(intersections) if
                     coordinate[0] != 0 or coordinate[1] != 0]

        return min(distances)

    def get_minimum_steps(self) -> int:
        intersections = self.get_intersection()

        steps = [self.places_first[intersection] + self.places_second[intersection] for intersection in intersections
                 if intersection != (0, 0)]

        return min(steps)

    def get_intersection(self) -> set:
        return set(self.places_first.keys()) \
            .intersection(set(self.places_second))


class CoordinateCalculator:

    def calculate_coordinates(self, wires: Wire):
        coordinates = {'x': 0, 'y': 0, 's': 0}
        places = {}

        for wire in wires.moves:
            if wire.direction == 'U':
                self.add_range(coordinates, 'x', wire.steps, True, places)

            if wire.direction == 'D':
                self.add_range(coordinates, 'x', wire.steps, False, places)

            if wire.direction == 'R':
                self.add_range(coordinates, 'y', wire.steps, True, places)

            if wire.direction == 'L':
                self.add_range(coordinates, 'y', wire.steps, False, places)

        return places

    def add_range(self, coordinates: dict, coord: str, value: int, positive: bool, places: dict):
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
