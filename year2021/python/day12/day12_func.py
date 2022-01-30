from collections import defaultdict


class PathFinder:

    def __init__(self, input_lines):
        connections = defaultdict(set)

        for line in input_lines:
            start, end = line.split('-')
            connections[start].add(end)
            connections[end].add(start)

        self.connections = connections

    def get_distinct_paths(self) -> set:
        distinct_paths = set()
        self.find_distinct_paths(self.connections,
                                 "start", {"start"}, distinct_paths, ["start"])
        return distinct_paths

    def find_distinct_paths(self, connections, point: str,
                            visited: set, distinct_paths: set, path: list):
        directions_from_start = connections[point]

        for direction in directions_from_start:

            if direction == "start":
                continue

            if not direction.isupper() and direction in visited:
                continue

            path_copy = path.copy()
            path_copy.append(direction)

            if direction == "end":
                distinct_paths.add(tuple(path_copy))
            else:
                visited_copy = visited.copy()
                visited_copy.add(direction)
                self.find_distinct_paths(connections, direction, visited_copy,
                                         distinct_paths, path_copy)

    def small_cave_step(self, connections, direction: str, visited: set,
                        distinct_paths: set, path: list):

        path_copy = path.copy()
        path_copy.append(direction)

        self.find_distinct_paths_two_visits(connections, direction, visited.copy(),
                                            distinct_paths, path_copy, True)

    def get_distinct_paths_two_visits(self) -> set:
        distinct_paths = set()
        self.find_distinct_paths_two_visits(self.connections,
                                            "start", {"start"}, distinct_paths, ["start"], False)
        return distinct_paths

    def find_distinct_paths_two_visits(self, connections, point: str,
                                       visited: set, distinct_paths: set, path: list,
                                       small_cave_visit: bool):
        directions_from_start = connections[point]

        for direction in directions_from_start:

            if direction == "start":
                continue

            if not direction.isupper():
                if not small_cave_visit and direction != "end":
                    self.small_cave_step(connections, direction, visited, distinct_paths, path)
                if direction in visited:
                    continue

            path_copy = path.copy()
            path_copy.append(direction)

            if direction == "end":
                distinct_paths.add(tuple(path_copy))
            else:
                visited_copy = visited.copy()
                visited_copy.add(direction)
                self.find_distinct_paths_two_visits(connections, direction, visited_copy,
                                                    distinct_paths, path_copy, small_cave_visit)
