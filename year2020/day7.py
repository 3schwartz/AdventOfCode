import common
import re

lines = common.get_lines("day7_data.txt")

bags = {}

for line in lines:
    colour = re.match(r".*(?= bags contain)", line)[0]
    bags[colour] = re.findall(r"(\d+?) (.+?) bags?", line)


def has_shiny(colour: str):
    if colour == "shiny gold":
        return True
    else:
        return any(has_shiny(c) for _, c in bags[colour])


shiny_bag_count = -1
for bag in bags:
    if has_shiny(bag):
        shiny_bag_count += 1

print(f"Part 1: {shiny_bag_count}")


def bag_calculator(colour: str):
    return 1 + sum(list(int(number) * bag_calculator(c) for number, c in bags[colour]))


print(f"Part 2: {bag_calculator('shiny gold') - 1}")
