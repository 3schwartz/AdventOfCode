import common

lines = common.get_lines("day2_data.txt")

within_validators = 0
exactly_once = 0
for line in lines:
    restrictions, password = line.split(': ')
    validators, char_rule = restrictions.split(' ')
    first_validator, second_validator = [int(validator) for validator in validators.split('-')]

    if first_validator <= password.count(char_rule) <= second_validator:
        within_validators += 1

    if (password[first_validator-1] == char_rule) | (password[second_validator-1] == char_rule):
        if password[first_validator-1] != password[second_validator-1]:
            exactly_once += 1

print(f"part 1: {within_validators}")
print(f"Part 2: {exactly_once}")
