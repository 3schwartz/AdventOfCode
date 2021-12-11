import unittest
from itertools import groupby
from collections import Counter, defaultdict, deque
from typing import Tuple


class SyntaxValidator:

    def __init__(self):
        self.openers = ['[', '(', '{', '<']
        self.closers = [']', ')', '}', '>']
        self.openersLookup = {"]": "[", ")": "(", "}": "{", ">": "<"}

    def isIncomplete(self, line: str):
        count = Counter(line)

        opened = 0
        closed = 0
        for syntax in count:
            if syntax in self.openers:
                opened += count[syntax]
            if syntax in self.closers:
                closed += count[syntax]

        return opened != closed

    def isCorupted(self, line: str):
        chunks = defaultdict(deque)

        for position, syntax in enumerate(line):
            if syntax in self.openers:
                chunks[syntax].append(position)
                continue

            lookup = self.openersLookup[syntax]
            if len(chunks[lookup]) == 0:
                raise Exception(f"Positiom {position} symbol {syntax} has no opening")
            lastPosition = chunks[lookup].pop()

            for otherOpeners in self.openers:
                if otherOpeners == lookup:
                    continue
                if len(chunks[otherOpeners]) == 0:
                    continue
                if chunks[otherOpeners][-1] > lastPosition:
                    return True, syntax
        return False, ""


class TestDay10(unittest.TestCase):

    def test_is_corrupted(self):
        # Arrange
        incomplete = "{([(<{}[<>[]}>{[]{[(<()>"
        validator = SyntaxValidator()

        # Act
        isIncomplete = validator.isCorupted(incomplete)

        # Arrange
        self.assertEqual(True, isIncomplete[0])
        self.assertEqual('}', isIncomplete[1])

    def test_not_is_corrupted(self):
        # Arrange
        incomplete = "[({(<(())[]>[[{[]{<()<>>"
        validator = SyntaxValidator()

        # Act
        isIncomplete = validator.isCorupted(incomplete)

        # Arrange
        self.assertEqual(False, isIncomplete[0])
        self.assertEqual("", isIncomplete[1])

