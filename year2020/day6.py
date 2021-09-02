import common
from collections import Counter

lines = common.get_lines("day6_data.txt")

groups = []
group = []
for line in lines:
    if line == "":
        groups.append(group)
        group = []
    else:
        group.append(line)
groups.append(group)

answers = 0
for group in groups:
    unique_answers = set()

    for answer in group:
        unique_answers = unique_answers.union(answer)

    answers += len(unique_answers)

print(f"Part 1: {answers}")

total = 0
for group in groups:
    counter = Counter(''.join(group))
    total += Counter(counter.values())[len(group)]

print(f"Part 2: {total}")