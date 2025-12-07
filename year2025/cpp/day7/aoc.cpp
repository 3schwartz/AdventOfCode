#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>

using std::map;
using std::pair;
using std::set;
using std::stoll;
using std::vector;

int part1(vector<string> lines)
{
    set<pair<int, int>> beam;
    set<pair<int, int>> splitters;

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

    int row_max = lines.size();
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

int part2(vector<string> lines)
{
    return -1;
}
