#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>

using std::map;
using std::pair;
using std::set;
using std::stoll;
using std::vector;

struct Start
{
    set<pair<int, int>> beam;
    set<pair<int, int>> splitters;
    int row_max;

    Start(vector<string> lines)
    {
        for (int row = 0; row < lines.size(); row++)
        {
            string line = lines[row];
            for (int col = 0; col < line.size(); col++)
            {
                char c = line[col];
                switch (c)
                {
                case '^':
                    splitters.insert({row, col});
                    break;
                case 'S':
                    beam.insert({row, col});
                    break;
                default:
                    break;
                }
            }
        }

        row_max = lines.size();
    }
};

int part1(vector<string> lines)
{

    Start start(lines);
    set<pair<int, int>> beam = std::move(start.beam);
    set<pair<int, int>> splitters = std::move(start.splitters);
    int row_max = start.row_max;

    int row_current = 0;
    int split_count = 0;
    while (row_current < row_max)
    {
        set<pair<int, int>> new_beam;
        for (auto &pos : beam)
        {
            pair<int, int> below = {pos.first + 1, pos.second};
            if (splitters.contains(below))
            {
                split_count++;
                new_beam.insert({below.first, below.second - 1});
                new_beam.insert({below.first, below.second + 1});
            }
            else
            {
                new_beam.insert(below);
            }
        }
        beam = new_beam;
        row_current++;
    }
    return split_count;
}

long long get_total_timeline(
    pair<int, int> pos,
    set<pair<int, int>> &splitters,
    map<pair<int, int>, long long> &timeline_cache,
    int row_max)
{
    if (timeline_cache.contains(pos))
    {
        return timeline_cache[pos];
    }

    int next_row = pos.first + 1;
    if (next_row == row_max)
    {
        return 1;
    }

    long long total_timeline = 0;

    pair<int, int> below = {next_row, pos.second};
    if (splitters.contains(below))
    {
        total_timeline += get_total_timeline(
            {below.first, below.second - 1},
            splitters,
            timeline_cache,
            row_max);
        total_timeline += get_total_timeline(
            {below.first, below.second + 1},
            splitters,
            timeline_cache,
            row_max);
    }
    else
    {
        total_timeline += get_total_timeline(
            below,
            splitters,
            timeline_cache,
            row_max);
    }
    timeline_cache[pos] = total_timeline;

    return total_timeline;
}

long long part2(vector<string> lines)
{
    Start start(lines);
    set<pair<int, int>> beam = std::move(start.beam);
    set<pair<int, int>> splitters = std::move(start.splitters);
    int row_max = start.row_max;

    long long total_timeline = 0;
    map<pair<int, int>, long long> timeline_cache;
    for (auto &pos : beam)
    {
        total_timeline += get_total_timeline(
            pos,
            splitters,
            timeline_cache,
            row_max);
    }
    return total_timeline;
}
