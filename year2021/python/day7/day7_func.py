from abc import ABC, abstractmethod
from collections import Counter


class Minimizer(ABC):

    @abstractmethod
    def getLowestFuel(self, crabsHorizontalPosition: str) -> int:
        pass

    def getLowestFuelWithCustomStep(self, crabsHorizontalPosition: str, step) -> int:
        ints = [int(number) for number in crabsHorizontalPosition.split(",")]
        count = Counter(ints)

        fuelOnIndex = [0] * (max(ints) + 1)

        for i in range(max(ints) + 1):
            fuel = 0
            for position in count:
                fuel += step(abs(i - position)) * count[position]
            fuelOnIndex[i] = fuel

        minFuel = min(fuelOnIndex)

        return minFuel



class BinomialMinimizer(Minimizer):

    def getLowestFuel(self, crabsHorizontalPosition: str) -> int:
        minFuel = self.getLowestFuelWithCustomStep(
            crabsHorizontalPosition,
            lambda difference: int(difference * (difference + 1) / 2))
        return minFuel


class StepMinimizer(Minimizer):

    def getLowestFuel(self, crabsHorizontalPosition: str) -> int:
        minFuel = self.getLowestFuelWithCustomStep(
            crabsHorizontalPosition,
            lambda difference: difference)
        return minFuel