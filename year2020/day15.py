from collections import defaultdict, deque

starting = [19, 0, 5, 1, 10, 13]

def get_spoken(turns: int):
    numbers = defaultdict(deque)

    for i, number in enumerate(starting):
        numbers[number].append(i)

    start = len(starting)
    spoken = starting[-1]

    for turn in range(start, turns):
        if len(numbers[spoken]) > 1:
            spoken = numbers[spoken][-1] - numbers[spoken][-2]
        else:
            spoken = 0
        numbers[spoken].append(turn)

    return spoken


print(f"Part 1: {get_spoken(2020)}")
print(f"Part 1: {get_spoken(30000000)}")