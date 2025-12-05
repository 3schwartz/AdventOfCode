#include "../common/common.h"
#include "aoc.h"
#include <map>
#include <set>

using std::map;
using std::pair;
using std::set;
using std::size_t;

bool is_valid_roll(const set<pair<int, int>> &grid, int x, int y)
{
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

    return rolls_count < 4;
}

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

        valid_rolls += is_valid_roll(grid, x, y) ? 1 : 0;
    }

    size_t size_before = grid.size();
    while (true)
    {
        set<pair<int, int>> new_grid;

        for (const auto &pos : grid)
        {
            int x = pos.first;
            int y = pos.second;

            if (!is_valid_roll(grid, x, y))
            {
                new_grid.insert({x, y});
            }
        }

        if (grid.size() == new_grid.size())
        {
            break;
        }

        grid = std::move(new_grid);
    };

    cout << "Part 1: " << valid_rolls << endl;
    cout << "Part 2: " << size_before - grid.size() << endl;

    return 0;
}
