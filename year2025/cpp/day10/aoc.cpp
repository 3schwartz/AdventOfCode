#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>
#include <queue>
#include <z3++.h>

using std::map;
using std::pair;
using std::queue;
using std::set;
using std::stoll;
using std::vector;

using z3::context;
using z3::expr;
using z3::expr_vector;
using z3::model;
using z3::optimize;
using z3::sat;
using z3::sum;

long long Machine::solveJoltage()
{
    context c;
    optimize opt(c);
    vector<expr> button_vars;

    for (size_t i = 0; i < buttons.size(); i++)
    {
        expr var = c.int_const(("b_" + std::to_string(i)).c_str());
        button_vars.push_back(var);
        opt.add(var >= 0);
    }

    for (size_t i = 0; i < joltages.size(); i++)
    {
        z3::expr_vector relevant_button_vars(c);
        for (size_t j = 0; j < buttons.size(); j++)
        {
            if (!buttons[j].contains(i))
            {
                continue;
            }
            relevant_button_vars.push_back(button_vars[j]);
        }
        expr s = (relevant_button_vars.size() == 0) ? c.int_val(0) : sum(relevant_button_vars);

        opt.add(s == c.int_val(joltages[i]));
    }
    expr_vector all(c);
    for (auto &button_var : button_vars)
    {
        all.push_back(button_var);
    }
    expr total = (all.size() == 0) ? c.int_val(0) : sum(all);
    opt.minimize(total);
    if (opt.check() != sat)
    {
        throw std::runtime_error("No solution found");
    }

    model m = opt.get_model();
    long long total_presses = 0;
    for (auto &button : button_vars)
    {
        z3::expr val = m.eval(button, true);

        if (!val.is_numeral())
            throw std::runtime_error("Non-numeric model value");

        total_presses += val.get_numeral_int64();
    }
    return total_presses;
}

struct State
{
    int light_diagram;
    long long steps;
};

struct StatePresses
{
    unsigned __int128 joltage;
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

using u128 = unsigned __int128;

bool anyByteIsBigger(unsigned __int128 a, unsigned __int128 b)
{
    for (size_t i = 0; i < 16; ++i)
    {
        unsigned __int128 byte_a = (a >> (i * 8)) & 0xFF;
        unsigned __int128 byte_b = (b >> (i * 8)) & 0xFF;
        if (byte_a > byte_b)
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
        set<int> button_set;
        for (auto &c : split(part.substr(1, part.size() - 2), ','))
        {
            button_set.insert(std::stoi(c));
        }
        buttons.push_back(button_set);
    }

    string s_j = s.substr(start_joltage + 1, s.size() - start_joltage - 2);
    vector<string> split_j = split(s_j, ',');
    joltage = 0;
    for (size_t i = 0; i < split_j.size(); ++i)
    {
        joltages.push_back(std::stoi(split_j[i]));
        unsigned __int128 val = std::stoull(split_j[i]);
        joltage += (val << (i * 8));
    }
    if (split_j.size() > 16)
    {
        throw std::runtime_error("Joltage too big");
    }
}

#include <unordered_set>

struct U128Hash
{
    size_t operator()(unsigned __int128 x) const noexcept
    {
        uint64_t lo = (uint64_t)x;
        uint64_t hi = (uint64_t)(x >> 64);
        return lo ^ (hi * 0x9e3779b97f4a7c15ULL);
    }
};

long long Machine::findFewestPresses()
{
    std::unordered_set<unsigned __int128, U128Hash> visited;
    queue<StatePresses> q;
    q.push({0, 0});

    while (!q.empty())
    {
        StatePresses current = q.front();
        q.pop();
        long long next_press = current.presses + 1;
        for (auto &button : buttons)
        {
            unsigned __int128 new_joltage = current.joltage;
            for (int pos : button)
            {
                new_joltage += ((unsigned __int128)1 << (pos * 8));
            }
            if (new_joltage == joltage)
            {
                return next_press;
            }
            if (anyByteIsBigger(new_joltage, joltage))
            {
                continue;
            }
            if (visited.insert(new_joltage).second)
            {
                q.push({new_joltage, next_press});
            }
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
        long long mPresses = machine.solveJoltage();
        presses += mPresses;
    }

    return presses;
}
