from year2021.python.day4.day4_func import *

input = [stack for stack in open('../../data/day4_data.txt').read().split('\n\n')]
game = BingoGame()

game.setUpGame(input)

score = game.winGame()

print(f"Part 1: {score}")

scoreWhenLose = game.loseGame()

print(f"Part 2: {scoreWhenLose}")