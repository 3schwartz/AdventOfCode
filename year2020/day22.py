import os
import sys

players = open(os.path.join(sys.path[0], 'day22_data.txt')).read().strip().split('\n\n')
player1, player2 = [[int(line) for line in player.split('\n')[1:]] for player in players]

while len(player1) > 0 and len(player2) > 0:
    c1, c2 = player1.pop(0), player2.pop(0)
    if c1 > c2:
        player1.extend([c1, c2])
    else:
        player2.extend([c2, c1])

winner = player1 if len(player1) > 0 else player2

print(f"Part 1: {sum(((i + 1) * card for i, card in enumerate(winner[::-1])))}")


def recursive_war(p1: list, p2: list, visited: set):
    while len(p1) > 0 and len(p2) > 0:
        visit = (tuple(p1), tuple(p2))
        if visit in visited:
            return 1, p1
        visited.add(visit)

        card1, card2 = p1.pop(0), p2.pop(0)

        if len(p1) >= card1 and len(p2) >= card2:
            win, _ = recursive_war(p1[:card1], p2[:card2], set())
        else:
            win = 1 if card1 > card2 else 0

        if win == 1:
            p1.extend([card1, card2])
        else:
            p2.extend([card2, card1])
    return (1, p1) if len(p1) > 0 else (0, p2)


player1, player2 = [[int(line) for line in player.split('\n')[1:]] for player in players]

_, cards = recursive_war(player1, player2, set())

print(f"Part 2: {sum((i+1) * card for i, card in enumerate(cards[::-1]))}")
