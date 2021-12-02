from abc import abstractmethod, ABC
from typing import List


class Sonar(ABC):

    @abstractmethod
    def GetDebtCount(self, debts: List[int]) -> int:
        pass


class SonarSingle(Sonar):

    def GetDebtCount(self, debts: List[int]):
        count = 0
        for i in range(1, len(debts)):

            if debts[i] > debts[i - 1]:
                count += 1
        return count


class SonarWindow(Sonar):

    def GetDebtCount(self, debts: List[int]) -> int:
        countSlide = 0
        for i in range(len(debts) - 3):
            if sum(debts[i + 1:i + 4]) > sum(debts[i:i + 3]):
                countSlide += 1
        return countSlide