from typing import List


class RateFinder:

    def CO2Oxygen(self, report: List[List[int]]):
        co2 = self.CO2Finder(report)
        oxygen = self.OxygenFinder(report)

        co2Binary = self.BinrayToDecimal(co2)
        oxygenBinary = self.BinrayToDecimal(oxygen)

        return co2Binary * oxygenBinary

    def Consumption(self, report: List[List[int]]) -> int:
        gamma = self.GammaRateFinder(report)
        epsilon = self.EpsilonRateFinder(report)

        gammaBinary = self.BinrayToDecimal(gamma)
        epsilonBinary = self.BinrayToDecimal(epsilon)

        return gammaBinary * epsilonBinary

    def CO2Finder(self, report: List[List[int]]) -> str:
        return self.ComplexFinder(report, lambda remaining, idx: sum(list(zip(*remaining))[idx]) < len(remaining) / 2, 0)

    def OxygenFinder(self, report: List[List[int]]) -> str:

        return self.ComplexFinder(report, lambda remaining, idx: sum(list(zip(*remaining))[idx]) > len(remaining) / 2, 1)

    def ComplexFinder(self, report: List[List[int]], condition, default: int):
        remaining = report.copy()
        idx = 0

        while True:
            within = list()

            bit = 1 if condition(remaining, idx) else 0
            for report in remaining:
                if report[idx] == bit:
                    within.append(report)

            if len(within) == 2:
                if within[0][idx + 1] == default:
                    remaining = within[0]
                else:
                    remaining = within[1]
                break
            elif len(within) == 1:
                remaining = within[0]
                break
            else:
                remaining = within

            idx += 1

        return "".join(str(bit) for bit in remaining)

    def GammaRateFinder(self, report: List[List[int]]) -> str:
        return self.SimpleFinder(report, lambda rep: sum(rep) > len(rep) / 2)

    def EpsilonRateFinder(self, report: List[List[int]]) -> str:

        return self.SimpleFinder(report, lambda rep: sum(report) < len(report) / 2)

    def SimpleFinder(self, report: List[List[int]], condition):
        rate = []
        for report in zip(*report):
            rate.append("1" if condition(report) else "0")
        return "".join(rate)

    def BinrayToDecimal(self, binrayString: str) -> int:
        return int(binrayString, 2)


class ReportGenerator:

    def DiagnosticReader(self, line: str) -> List[int]:
        return [int(code) for code in line]
