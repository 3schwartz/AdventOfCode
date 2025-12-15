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

const string START = "you";
const string END = "out";

struct State
{
    string position;
    string visited;
};

long long part1(const vector<string> &lines)
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

    queue<State> q;
    q.push({START, START});

    int path_count = 0;

    while (!q.empty())
    {
        State current = q.front();
        q.pop();
        if (current.position == END)
        {
            path_count++;
        }
        for (const string &neighbor : edges[current.position])
        {
            if (current.visited.find(neighbor) != string::npos)
            {
                continue;
            }
            string new_visited = current.visited + ',' + neighbor;
            q.push({neighbor, new_visited});
        }
    }

    return path_count;
}

long long part2(const vector<string> &lines)
{
    return -1;
}
