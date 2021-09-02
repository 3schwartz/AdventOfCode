from itertools import groupby


def rule_increasing(s):
    return [a <= b for a, b in zip(s, s[1:])]


def rule_two_sequentially_increasing(s):
    return [a == b for a, b in zip(s, s[1:])]


def rule_count_groups_of_two(s):
    return [len(list(k)) == 2 for _, k in groupby(s)]


def get_rules():
    return [
        lambda s: all(rule_increasing(s)),
        lambda s: any(rule_two_sequentially_increasing(s)),
        lambda s: any(rule_count_groups_of_two(s))
    ]


def sequence_pass_rules(s, rules):
    return all(f(str(s)) for f in rules)
