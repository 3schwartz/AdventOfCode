from year2021.python.day6.day6_func import *

inputStr = open('../../data/day6_data.txt').readline()

laternFishes = LaternFishCreator.getLaternFishes(inputStr)
spawn = FishSpawn()

numberOfFishes80Days =  spawn.spawn(inputStr, 80)

print(f"Part 1: {numberOfFishes80Days}")


numberOfFishes256Days = spawn.spawn(inputStr, 256)

print(f"Part 2: {numberOfFishes256Days}")