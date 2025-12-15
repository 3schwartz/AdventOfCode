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
using std::tuple;
using std::vector;

const string START_PART_1 = "you";
const string START_PART_2 = "svr";
const string END = "out";

map<string, set<string>>
parse(const vector<string> &lines)
{
    map<string, set<string>> edges;
    for (const string &line : lines)
    {
        auto parts = split(line, ": ");
        if (parts.size() != 2)
        {
            throw std::runtime_error("Invalid line: " + line);
        }
        auto neighbors_list = split(parts[1], ' ');
        set<string> neighbors;
        for (const string &neighbor : neighbors_list)
        {
            neighbors.insert(neighbor);
        }
        edges[parts[0]] = neighbors;
    }

    return edges;
}

struct StatePart1
{
    string position;
    string visited;

    bool satisfiesEndCondition() const { return true; }
};

struct StatePart2
{
    string position;
    string visited;

    bool satisfiesEndCondition() const
    {
        return visited.find("fft") != string::npos &&
               visited.find("dac") != string::npos;
    }
};

template <class State>
long long findTotalPaths(const map<string, set<string>> &edges, State initial_state)
{
    queue<State> q;
    q.push(std::move(initial_state));

    long long path_count = 0;

    while (!q.empty())
    {
        State current = std::move(q.front());
        q.pop();

        if (current.position == END)
        {
            if (current.satisfiesEndCondition())
                ++path_count;
            continue;
        }

        for (const string &neighbor : edges.at(current.position))
        {
            if (current.visited.find(neighbor) != string::npos)
                continue;

            State next = current;
            next.position = neighbor;
            next.visited += ',' + neighbor;
            q.push(std::move(next));
        }
    }

    return path_count;
}

long long findTotalPathsDept(
    const map<string, set<string>> &edges,
    map<tuple<string, bool, bool>, long long> &memo,
    string visited,
    string position)
{
    const bool has_fft = visited.find("fft") != string::npos;
    const bool has_dac = visited.find("dac") != string::npos;
    tuple<string, bool, bool> state_key = std::make_tuple(position, has_fft, has_dac);
    if (memo.contains(state_key))
    {
        return memo.at(state_key);
    }
    if (position == END)
    {
        return has_fft && has_dac ? 1 : 0;
    }
    long long path_count = 0;
    for (const string &neighbor : edges.at(position))
    {
        if (visited.find(neighbor) != string::npos)
        {
            continue;
        }
        string new_visited = visited + ',' + neighbor;
        path_count += findTotalPathsDept(edges, memo, new_visited, neighbor);
    }
    memo[state_key] = path_count;
    return path_count;
}

long long part1(const vector<string> &lines)
{
    auto edges = parse(lines);
    StatePart1 initial_state{START_PART_1, START_PART_1};
    return findTotalPaths(edges, initial_state);
}

long long part2(const vector<string> &lines)
{
    auto edges = parse(lines);
    // StatePart2 initial_state{START_PART_2, START_PART_2};
    // return findTotalPaths(edges, initial_state);
    map<tuple<string, bool, bool>, long long> memo;
    return findTotalPathsDept(edges, memo, START_PART_2, START_PART_2);
}
