from year2019.python.day5.day5_func import *

instructions = open('day5_data.txt').read().split(',')

input_parameter = 1
output = []
i = 0
while i < len(instructions):
    instruction = instructions[i]

    prefixed_instruction = add_zero(instruction, 5)
    opcode, opcode_list = get_opcode(prefixed_instruction)

    if opcode == '03':
        parameter = i + 1
        if opcode_list[2] == '0':
            parameter = int(instructions[i + 1])

        opcode_three_at_input_at_position(instructions, parameter, input_parameter)
        i += 2
        continue

    if opcode == '04':
        parameter = int(instructions[i + 1])
        if opcode_list[2] == '0':
            parameter = int(instructions[int(instructions[i + 1])])

        output.append(parameter)
        i += 2
        continue

    if opcode == '99':
        break

    position = get_position(opcode_list[0], i, instructions)

    actions = get_actions(opcode_list[-3:0:-1], i, instructions)

    if opcode == '01':
        instructions[position] = str(actions[0] + actions[1])
    elif opcode == '02':
        instructions[position] = str(actions[0] * actions[1])
    else:
        raise OSError(f"Opcode was {opcode}")

    i += 4

print(f"Part 1: {output[-1]}")
