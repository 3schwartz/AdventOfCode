import os
import sys

with open(os.path.join(sys.path[0], 'day19_data.txt')) as file:
    rules, messages = [x.splitlines() for x in file.read().split("\n\n")]


def create_rules(rules):
    rule_dict = {}
    for rule in rules:
        idx, rule = rule.split(': ')

        inner_rules = []

        if rule[0] == '"':
            inner_rules.extend(rule[1:-1])
        else:
            for inner_rule in rule.split(" | "):
                inner_rules.append([int(inner) for inner in inner_rule.split(' ')])

        rule_dict[int(idx)] = inner_rules

    return rule_dict


def rule_matches_message(message: list, rule: list, rules: dict):
    if len(message) == 0 or len(rule) == 0:
        return len(message) == len(rule)

    c = rule.pop()
    if isinstance(c, str):
        if message[0] == c:
            return rule_matches_message(message[1:], rule.copy(), rules)
    else:
        for inner in rules[c]:
            if rule_matches_message(message, rule + list(reversed(inner)), rules):
                return True
    return False


rules = create_rules(rules)

print(f"Part 1: {sum(rule_matches_message(message, list(reversed(rules[0][0])), rules) for message in messages)}")

rules[8] = [[42], [42, 8]]
rules[11] = [[42, 31], [42, 11, 31]]

print(f"Part 2: {sum(rule_matches_message(message, list(reversed(rules[0][0])), rules) for message in messages)}")