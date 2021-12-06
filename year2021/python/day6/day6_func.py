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

    def spawn(self, fishesInit: List[int], days: int):
        fishes = fishesInit.copy()
        for i in range(days):
            fishes += [fishes.pop(0)]
            fishes[6] += fishes[8]
        return sum(fishes)
