lines = open('../../data/day2_data.txt').read().split(',')
lines = [int(line) for line in lines]


def int_code_program(instructions: list):
    for i in range(0, len(instructions), 4):
        intCode = instructions[i]

        if intCode == 1:
            instructions[instructions[i + 3]] = instructions[instructions[i + 1]] + instructions[instructions[i + 2]]

        if intCode == 2:
            instructions[instructions[i + 3]] = instructions[instructions[i + 1]] * instructions[instructions[i + 2]]

        if intCode == 99:
            break
    return instructions[0]


linesCopy = lines.copy()

linesCopy[1] = 12
linesCopy[2] = 2

print(f"Part 1: {int_code_program(linesCopy)}")

found = False
for i in range(100):
    for j in range(100):
        linesCopy = lines.copy()

        linesCopy[1] = i
        linesCopy[2] = j

        positionZeroAtHalt = int_code_program(linesCopy)

        if positionZeroAtHalt == 19690720:
            found = True

        if found:
            break
    if found:
        break

print(f"Part 2: {100 * i + j}")
