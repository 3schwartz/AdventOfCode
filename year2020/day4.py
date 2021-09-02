import common
import re

lines = common.get_lines('day4_data.txt')

keys = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

passport = {}
valid_passports = []

for line in lines:
    if line != '':
        passport = {**passport, **{item.split(':')[0]: item.split(':')[1] for item in line.split(' ')}}
    else:
        if all(key in passport for key in keys):
            valid_passports.append(passport)
        passport = {}
if all(key in passport for key in keys):
    valid_passports.append(passport)

print(f"Part 1: {len(valid_passports)}")

password_following_rules = 0

for passport in valid_passports:

    valid = True

    for key, value in passport.items():
        if key == 'byr':
            if not 1920 <= int(value) <= 2002:
                valid = False

        if key == 'iyr':
            if not 2010 <= int(value) <= 2020:
                valid = False

        if key == 'eyr':
            if not 2020 <= int(value) <= 2030:
                valid = False

        if key == 'hgt':
            if value[-2:] == 'cm' and 150 <= int(value[:-2]) <= 193:
                pass
            elif value[-2:] == 'in' and 59 <= int(value[:-2]) <= 76:
                pass
            else:
                valid = False

        if key == 'hcl':
            if not re.match('^#[0-9a-f]{6}', value):
                valid = False

        if key == 'ecl':
            if value not in 'amb blu brn gry grn hzl oth'.split(' '):
                valid = False

        if key == 'pid':
            if not re.match('^[0-9]{9}$', value):
                valid = False

    if valid:
        password_following_rules += 1

print(f"Part 2 {password_following_rules}")