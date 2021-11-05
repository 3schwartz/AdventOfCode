#ifndef DAY_THREE_FUNC_H
#define DAY_THREE_FUNC_H

#include <vector>
#include <string>
#include <algorithm>
#include <map>
#include <set>
#include <tuple>
#include <cmath>
#include <iostream>

void addSteps(std::map<char, int> &coordinates, char coord, int multiplier, int value, std::map<std::tuple<int, int>, int> &places, int steps);

std::map<std::tuple<int, int>, int> getCoordinates(std::vector<std::string> wire);

template <class T, class U>
std::set<T> getSetFromKeys(std::map<T,U> mapWithKeys);

int getMinimumDistance(std::vector<std::string> wireOneInput, std::vector<std::string> wireTwoInput);

int getMinimumSteps(std::vector<std::string> wireOneInput, std::vector<std::string> wireTwoInput);

#endif