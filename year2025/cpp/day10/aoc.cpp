#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>
#include <queue>

using std::map;
using std::pair;
using std::queue;
using std::set;
using std::stoll;
using std::vector;

struct State
{
    int light_diagram;
    long long steps;
};

struct StatePresses
{
    map<int, int> joltage_presses;
    long long presses;
};

bool anyJoltageIsBigger(const map<int, int> &a, const map<int, int> &b)
{
    for (const auto &[key, value] : b)
    {
        if (a.find(key) != a.end() && a.at(key) > value)
        {
            return true;
        }
    }
    return false;
}

Machine::Machine(const string &s)
{
    light_diagram = 0;
    string s_l = s.substr(1, s.find(']') - 1);
    for (size_t i = 0; i < s_l.size(); ++i)
    {
        if (s_l[i] == '#')
        {
            light_diagram |= (1 << i);
        }
    }

    size_t start_button = s.find(']') + 2;
    size_t start_joltage = s.find('{');
    string s_b = s.substr(start_button, start_joltage - start_button - 1);
    vector<string> button_parts = split(s_b, ' ');
    for (const string &part : button_parts)
    {
        vector<int> button;
        for (auto &c : split(part.substr(1, part.size() - 2), ','))
        {
            button.push_back(std::stoi(c));
        }
        buttons.push_back(button);
    }

    string s_j = s.substr(start_joltage + 1, s.size() - start_joltage - 2);
    vector<string> split_j = split(s_j, ',');
    for (size_t i = 0; i < split_j.size(); ++i)
    {
        unsigned __int128 foo = 1;
        joltage[i] = std::stoi(split_j[i]);
    }
    if (joltage.size() > 128 / 8)
    {
        joltage[0] = 0;
    }
}

long long Machine::findFewestPresses()
{
    set<map<int, int>> visited;
    queue<StatePresses> q;
    q.push({map<int, int>(), 0});

    while (!q.empty())
    {
        StatePresses current = q.front();
        q.pop();
        if (!visited.insert(current.joltage_presses).second)
        {
            continue;
        }
        long long next_press = current.presses + 1;
        for (auto &button : buttons)
        {
            map<int, int> new_joltage = current.joltage_presses;
            for (int pos : button)
            {
                new_joltage[pos] += 1;
            }
            if (anyJoltageIsBigger(new_joltage, joltage))
            {
                continue;
            }
            if (new_joltage == joltage)
            {
                return next_press;
            }
            q.push({new_joltage, next_press});
        }
    }

    return -1;
}

long long Machine::findFewestSteps()
{
    set<int> visited;
    queue<State> q;
    q.push({0, 0});

    while (!q.empty())
    {
        State current = q.front();
        q.pop();
        if (!visited.insert(current.light_diagram).second)
        {
            continue;
        }
        long long next_steps = current.steps + 1;
        for (auto &button : buttons)
        {
            int new_diagram = current.light_diagram;
            for (int pos : button)
            {
                new_diagram ^= (1 << pos);
            }
            if (new_diagram == light_diagram)
            {
                return next_steps;
            }
            q.push({new_diagram, next_steps});
        }
    }

    return -1;
};

long long part1(const vector<string> &lines)
{
    long long steps = 0;
    for (const string &line : lines)
    {
        Machine machine = Machine(line);
        long long mSteps = machine.findFewestSteps();
        steps += mSteps;
    }

    return steps;
}

long long part2(const vector<string> &lines)
{
    long long presses = 0;
    for (const string &line : lines)
    {
        Machine machine = Machine(line);
        long long mPresses = machine.findFewestPresses();
        presses += mPresses;
    }

    return presses;
}
