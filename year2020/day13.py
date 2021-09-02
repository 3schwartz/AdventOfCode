import common

lines = common.get_lines("day13_data.txt")

earliest = int(lines[0])
busses = [int(x) for x in lines[1].split(',') if x != 'x']

wait = [-earliest % bus for bus in busses]

print(f"Part 1: {busses[wait.index(min(wait))] * min(wait)}")

mods = {int(bus): -i % int(bus) for i, bus in enumerate(lines[1].split(',')) if bus != 'x'}

busses = list(reversed(sorted(mods)))
inc = busses[0]
earliest = mods[inc]

for bus in busses[1:]:
    while earliest % bus != mods[bus]:
        earliest += inc
    inc *= bus

print(f"Part 2: {earliest}")
