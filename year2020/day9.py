import common

lines = [int(line) for line in common.get_lines("day9_data.txt")]

for index in range(25, len(lines)):
    subLines = lines[index-25: index]

    valid = False
    for first in subLines:
        for second in subLines:
            if first + second == lines[index]:
                valid = True
                break

        if valid:
            break

    if not valid:
        break

invalid = lines[index]

print(f"Part 1: {invalid}")

for i in range(len(lines)):
    for j in range(i, len(lines)):
        if sum(lines[i:j]) == invalid:
            print(f"Part 2: {min(lines[i:j]) + max(lines[i:j])}")
            break
        if sum(lines[i:j]) > invalid:
            break

