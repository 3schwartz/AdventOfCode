import common
from functools import reduce

lines = common.get_lines("day16_data.txt")

sections = []
current_section = []
for line in lines:
    if line == '':
        sections.append(current_section)
        current_section = []
    else:
        current_section.append(line)
sections.append(current_section)
classes, own_ticket, tickets = sections

possible = set()
for cls in classes:
    intervals = [interval_values.split('-') for interval_values in cls.split(': ')[1].split(' or ')]
    for interval in intervals:
        [possible.add(int(x)) for x in range(int(interval[0]), int(interval[1]) + 1)]

invalid_sum = 0
for ticket in tickets[1:]:
    numbers = ticket.split(',')

    invalid_sum += sum([int(x) for x in numbers if int(x) not in possible])

print(f"Part 1: {invalid_sum}")

limits = []
class_count = 20

for cls in classes:
    intervals = [range(int(x[0]), int(x[1]) + 1) for x in [x.split('-') for x in cls.split(': ')[1].split(' or ')]]
    limits.append(intervals)

OK = [[True for _ in range(class_count)] for _ in range(class_count)]

for ticket in tickets[1:]:
    numbers = ticket.split(',')

    if any([int(x) not in possible for x in numbers]):
        continue

    for i, number in enumerate(numbers):
        for j, limit in enumerate(limits):
            if not any([int(number) in lim for lim in limit]):
                OK[i][j] = False

class_idx = [None for _ in range(class_count)]

while True:
    for i in range(class_count):
        valid_limits = [j for j in range(class_count) if OK[i][j] and j not in class_idx]
        if len(valid_limits) == 1:
            class_idx[i] = valid_limits[0]
    if len([x for x in class_idx if x is not None]) == 20:
        break

idx = [i for i, x in enumerate(class_idx) if x < 6]

print(f"Part 2: {reduce((lambda x, y: x * y), [int(own_ticket[1].split(',')[i]) for i in idx])}")
