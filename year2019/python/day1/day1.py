from day1_func import get_fuel

lines = open('../../data/day1_data.txt').read().strip().split('\n')

fuels = [get_fuel(int(line)) for line in lines]

print(f'Part 1: {sum(fuels)}')

total_sum = 0

for line in lines:
    last_value = int(line)
    while True:
        last_value = get_fuel(last_value)
        if last_value <= 0:
            break
        total_sum += last_value

print(f"Part 2: {total_sum}")

