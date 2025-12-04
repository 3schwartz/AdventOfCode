#include "../common/common.h"
#include "aoc.h"
#include <map>
#include <set>

using std::map;
using std::pair;
using std::set;
using std::size_t;

int main()
{
    const vector<string> lines = read_lines("../../../../data/day4_data.txt");

    set<pair<int, int>> grid;

    for (size_t y = 0; y < lines.size(); ++y)
    {
        const string &line = lines[y];
        for (size_t x = 0; x < line.size(); ++x)
        {
            char c = line[x];
            if (c == '@')
            {
                grid.insert({x, y});
            }
        }
    }

    int x_max = lines[0].size();
    int y_max = lines.size();

    int valid_rolls = 0;

    for (const auto &pos : grid)
    {
        int x = pos.first;
        int y = pos.second;
        int rolls_count = 0;
        for (int dy = -1; dy <= 1; ++dy)
        {
            for (int dx = -1; dx <= 1; ++dx)
            {
                if (dx == 0 && dy == 0)
                    continue;
                int nx = x + dx;
                int ny = y + dy;
                if (grid.contains({nx, ny}))
                {
                    rolls_count++;
                }
            }
        }
        if (rolls_count < 4)
        {
            valid_rolls++;
        }
    }

    cout << "Part 1: " << valid_rolls << endl;

    return 0;
}
