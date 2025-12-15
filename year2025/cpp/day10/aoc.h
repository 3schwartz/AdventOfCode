#include <vector>
#include <string>
#include <map>
#include <set>

using std::map;
using std::set;
using std::string;
using std::vector;

#pragma once

long long part1(const vector<string> &lines);
long long part2(const vector<string> &lines);

struct Machine
{
    int light_diagram;
    vector<set<int>> buttons;
    unsigned __int128 joltage;
    vector<int> joltages;

    Machine(const string &s);

    long long findFewestSteps();
    long long findFewestPresses();
    long long solveJoltage();
};
