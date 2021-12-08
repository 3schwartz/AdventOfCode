from collections import defaultdict, deque

class Signal:

    def __init__(self, inputRow: str):
        signals, digits = inputRow.strip().split(" | ")
        self.signals = signals.split(' ')
        self.digits = digits.split(' ')

    def numberUniqueInDigits(self):
        numberUniqueInRow = 0
        for digit in self.digits:
            if len(digit) in [7, 4, 3, 2]:
                numberUniqueInRow += 1
        return numberUniqueInRow

    def outputNumber(self):
        numbers = defaultdict(deque)

        for signal in self.signals:
            numbers[len(signal)].append(set(signal))

        numberOrdered = [0] * 10

        one = numbers[2]
        numberOrdered[1] = one[0]

        four = numbers[4]
        numberOrdered[4] = four[0]

        seven = numbers[3]
        numberOrdered[7] = seven[0]

        eight = numbers[7]
        numberOrdered[8] = eight[0]

        three = [number for number in numbers[5] if set.intersection(number, one[0]) == one[0]]
        numberOrdered[3] = three[0]

        nine = [number for number in numbers[6] if set.intersection(number, four[0]) == four[0]]
        numberOrdered[9] = nine[0]

        zero = [number for number in numbers[6] if set.intersection(number, seven[0]) == seven[0] and number != nine[0]]
        numberOrdered[0] = zero[0]

        six = [number for number in numbers[6] if number not in [zero[0], nine[0]]]
        numberOrdered[6] = six[0]

        five = [number for number in numbers[5] if set.intersection(number, nine[0]) == number and number != three[0]]
        numberOrdered[5] = five[0]

        two = [number for number in numbers[5] if number not in [three[0], five[0]]]
        numberOrdered[2] = two[0]

        outNumber = 0
        for digit in self.digits:
            outNumber *= 10
            outNumber += numberOrdered.index(set(digit))

        return outNumber
                

