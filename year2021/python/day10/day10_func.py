from collections import defaultdict, deque
from typing import List


class Sentence:
    def __init__(self, isCorupted: bool, coruptedSyntax: str, chunksLeft: defaultdict(deque)):
        self.isCorupted = isCorupted
        self.coruptedSyntax = coruptedSyntax
        self.chunksLeft = chunksLeft

    def getIncompleteScore(self) -> int:
        positions = {}
        for key, items in self.chunksLeft.items():
            for item in items:
                positions[item] = key

        scoreLookup = {"(": 1, "[": 2, "{": 3, "<": 4}
        score = 0
        for key in sorted(positions, reverse=True):
            score = score * 5 + scoreLookup[positions[key]]
        return score


class SyntaxValidator:

    def __init__(self):
        self.openers = ['[', '(', '{', '<']
        self.closers = [']', ')', '}', '>']
        self.openersLookup = {"]": "[", ")": "(", "}": "{", ">": "<"}
        self.scoreLoopup = {")": 3, "]": 57, "}": 1197, ">": 25137}

    def isCorupted(self, line: str) -> Sentence:
        chunks = defaultdict(deque)

        for position, syntax in enumerate(line):
            if syntax in self.openers:
                chunks[syntax].append(position)
                continue

            lookup = self.openersLookup[syntax]
            if len(chunks[lookup]) == 0:
                return Sentence(True, syntax, chunks)

            lastPosition = chunks[lookup].pop()

            for otherOpeners in self.openers:
                if otherOpeners == lookup:
                    continue
                if len(chunks[otherOpeners]) == 0:
                    continue
                if chunks[otherOpeners][-1] > lastPosition:
                    return Sentence(True, syntax, chunks)
        return Sentence(False, "", chunks)

    def getScore(self, input: List[str]) -> int:
        lines = [self.isCorupted(line.strip()) for line in input]
        illegals = [self.scoreLoopup[illegal.coruptedSyntax] for illegal in lines if illegal.isCorupted]
        return sum(illegals)

    def getIncompleteScore(self, input: List[str]) -> int:
        lines = [self.isCorupted(line.strip()) for line in input]
        inCompleteScore = [incomplete.getIncompleteScore() for incomplete in lines if not incomplete.isCorupted]
        return sorted(inCompleteScore)[int(len(inCompleteScore) / 2)]
