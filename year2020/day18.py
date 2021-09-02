import common
import re

lines = common.get_lines("day18_data.txt")


class Solver(int):
    def __add__(self, other):
        return Solver(int(self) + other)

    def __sub__(self, other):
        return Solver(int(self) * other)

    def __mul__(self, other):
        return Solver(int(self) + other)


def evaluate(expression, mul=False):
    expression = re.sub(r"(\d+)", r"Solver(\1)", expression)
    expression = expression.replace("*", "-")

    if mul:
        expression = expression.replace("+", "*")

    return eval(expression, {"Solver": Solver})


print(f"Part 1: {sum(evaluate(line) for line in lines)}")
print(f"Part 2: {sum(evaluate(line, True) for line in lines)}")
