from typing import List
from collections import Counter

class FishCreator:

    @staticmethod
    def initFishes(intputStr: str) -> List[int]:
        fishes = [0] * 9
        initialFishes = Counter(intputStr.split(','))

        for fishKey in initialFishes:
            fishes[int(fishKey)] = initialFishes[fishKey]

        return fishes

class FishSpawn:
    def spawn(self, intputStr: str, days: int):
        fishes = self.initFishes(intputStr)
        for i in range(days):
            fishes += [fishes.pop(0)]
            fishes[6] += fishes[8]
        return sum(fishes)


class LaternFish:

    def __init__(self, timer=None):
        if timer:
            self.timer = timer
        else:
            self.timer = 8
        self.children = []

    def spawn(self):
        for fish in self.children:
            fish.spawn()

        if self.timer == 0:
            self.timer = 6
            self.children.append(LaternFish())

        self.timer -= 1

    def getLength(self):

        return 1 + sum([fish.getLength() for fish in self.children])


class LaternFishSpawn:

    def spawn(self, laternFishes: List[LaternFish], days: int) -> int:

        fishes = [LaternFish(fish.timer) for fish in laternFishes]

        for i in range(days):
            # newFishes = []
            for fish in fishes:
                fish.spawn()
                # newFishes.extend(fish.spawn())
            # fishes.extend(newFishes)

        return sum([fish.getLength() for fish in fishes])


class LaternFishCreator:

    @staticmethod
    def getLaternFishes(inputStri: str) -> List[LaternFish]:
        return [LaternFish(int(timer)) for timer in inputStri.split(',')]
