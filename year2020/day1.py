import common

lines = common.get_lines('day1_data.txt')

for i in lines:
    for j in lines:
        if int(i) + int(j) == 2020:
            print(f"Part 1: {int(i) * int(j)}")
        for k in lines:
            if int(i) + int(j) + int(k) == 2020:
                print(f"Part 2: {int(i) * int(j) * int(k)}")
