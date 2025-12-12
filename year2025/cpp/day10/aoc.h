#include <vector>
#include <string>
#include <map>

using std::map;
using std::string;
using std::vector;

#pragma once

long long part1(const vector<string> &lines);
long long part2(const vector<string> &lines);

struct Machine
{
    int light_diagram;
    vector<vector<int>> buttons;
    map<int, int> joltage;

    Machine(const string &s);

    long long findFewestSteps();
    long long findFewestPresses();
};
