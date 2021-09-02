def opcode_three_at_input_at_position(sequence: list, position: int, input: int):
    sequence[position] = input


def add_zero(sequence: str, zero_length: int) -> list:
    prefix_zeros = (['0'] * (zero_length - len(sequence)))
    prefix_zeros.extend([sequence])
    return ''.join(prefix_zeros)


def get_opcode(opcode_string):
    opcode_list = [code for code in opcode_string]
    opcode = ''.join(opcode_list[-2:])
    return opcode, opcode_list


def get_position(opcode_position, index, instructions):
    if opcode_position == '0':
        return int(instructions[index + 3])

    return index + 3


def get_actions(action_parameters, index, instructions):
    actions = [int(instructions[index + 1]), int(instructions[index + 2])]

    for j, c in enumerate(action_parameters):
        if c == '0':
            actions[j] = int(instructions[int(instructions[index + 1 + j])])

    return actions