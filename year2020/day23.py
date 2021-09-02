from collections import deque
import time

input_string = "327465189"

cups = deque(int(cup) for cup in input_string)
for _ in range(100):
    current = cups[0]

    cups.rotate(-1)
    cup1 = cups.popleft()
    cup2 = cups.popleft()
    cup3 = cups.popleft()

    destination = current - 1 or 9
    while destination in (cup1, cup2, cup3):
        destination = destination - 1 or destination + 8

    while cups[0] != destination:
        cups.rotate(-1)
    cups.rotate(-1)

    cups.append(cup1)
    cups.append(cup2)
    cups.append(cup3)

    while cups[0] != current:
        cups.rotate(-1)
    cups.rotate(-1)

while cups[0] != 1:
    cups.rotate(-1)
cups.popleft()

print(f"Part 1: {''.join(str(cup) for cup in cups)}")


class Cup:
    def __init__(self, cup):
        self.cup = cup
        self.left = None
        self.right = None


cups = list(int(cup) for cup in input_string) + list(range(len(input_string)+1, 1_000_001))
cups_dict = {}

tic = time.perf_counter()

last_cup = None
for i in cups:
    current_cup = Cup(i)
    cups_dict[i] = current_cup

    if last_cup is not None:
        last_cup.right = current_cup
        current_cup.left = last_cup

    last_cup = current_cup

first = cups_dict[cups[0]]
last_cup.right = first
first.left = last_cup

for i in range(10_000_000):
    current = first.cup

    cup1 = first.right
    cup2 = cup1.right
    cup3 = cup2.right

    first.right = cup3.right
    first.right.left = first

    destination = current - 1 or 1_000_000
    while destination in (cup1.cup, cup2.cup, cup3.cup):
        destination = destination - 1 or 1_000_000

    destination_cup = cups_dict[destination]

    cup3.right = destination_cup.right
    cup3.right.left = cup3

    destination_cup.right = cup1
    cup1.left = destination

    first = first.right

while first.cup != 1:
    first = first.right

print(f"Part 2: {first.right.cup * first.right.right.cup}")

toc = time.perf_counter()

print(f"Part 2 time : {toc - tic:0.4f} seconds")
