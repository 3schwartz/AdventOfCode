import common

lines = common.get_lines("day8_data.txt")


def run_game(sequence: list):
    acceleration = 0
    position = 0
    visited = set()

    while True:
        if position == len(lines) - 1:
            break

        if position in visited:
            break

        visited.add(position)

        task, value = sequence[position].split(' ')

        if task == 'acc':
            acceleration += int(value)
            position += 1
        if task == 'jmp':
            position += int(value)

        if task == 'nop':
            position += 1

    return acceleration, position


print(f"Part 1: {run_game(lines)[0]}")

for position in range(0, len(lines)):

    task, value = lines[position].split(' ')

    if task == 'jmp':
        task = 'nop'
    elif task == 'nop':
        task = 'jmp'

    line_copy = lines.copy()
    line_copy[position] = ' '.join([task, value])
    acceleration, position = run_game(line_copy)
    if position == len(lines) - 1:
        break

print(f"Part 2: {acceleration}")