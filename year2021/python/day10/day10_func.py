from collections import defaultdict, deque
from typing import Tuple, List, TextIO


class SyntaxValidator:

    def __init__(self):
        self.openers = ['[', '(', '{', '<']
        self.closers = [']', ')', '}', '>']
        self.openersLookup = {"]": "[", ")": "(", "}": "{", ">": "<"}
        self.scoreLoopup = {")": 3, "]": 57, "}": 1197, ">": 25137}

    def isCorupted(self, line: str) -> Tuple[bool, str]:
        chunks = defaultdict(deque)

        for position, syntax in enumerate(line):
            if syntax in self.openers:
                chunks[syntax].append(position)
                continue

            lookup = self.openersLookup[syntax]
            if len(chunks[lookup]) == 0:
                return True, syntax
                # raise Exception(f"Positiom {position} symbol {syntax} has no opening")
            lastPosition = chunks[lookup].pop()

            for otherOpeners in self.openers:
                if otherOpeners == lookup:
                    continue
                if len(chunks[otherOpeners]) == 0:
                    continue
                if chunks[otherOpeners][-1] > lastPosition:
                    return True, syntax
        return False, ""

    def getScore(self, input: TextIO) -> int:
        lines = [self.isCorupted(line.strip()) for line in input]
        illegals = [self.scoreLoopup[illegal[1]] for illegal in lines if illegal[0]]
        return sum(illegals)
