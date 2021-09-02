import common

lines = common.get_lines("day14_data.txt")

mask = None
memory = {}

for line in lines:
    instruction, value = line.split(' = ')
    if instruction == 'mask':
        mask = {i: m for i, m in enumerate(value) if m != 'X'}
    else:
        bit = list(format(int(value), 'b').zfill(36))
        for i in mask.keys():
            bit[i] = mask[i]
        memory[instruction] = int(''.join(bit), 2)

print(f"Part 1: {sum(memory.values())}")

memory = {}

for line in lines:
    instruction, value = line.split(' = ')

    if instruction == 'mask':
        mask = {i: m for i, m in enumerate(value) if m != '0'}
    else:
        address = list(format(int(instruction[4:-1]), 'b').zfill(36))

        for i in mask.keys():
            address[i] = mask[i]

        combinations = []
        for i in range(2 ** address.count('X')):
            combinations.append(list(format(int(i), 'b').zfill(address.count('X'))))

        for combination in combinations:
            bits = address.copy()

            idx = [i for i, x in enumerate(address) if x == 'X']

            for i, com in zip(idx, combination):
                bits[i] = com

            memory[int(''.join(bits), 2)] = int(value)

print(f"Part 2: {sum(memory.values())}")
