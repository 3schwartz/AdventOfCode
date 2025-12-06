#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>

using std::map;
using std::pair;
using std::set;
using std::stoll;
using std::vector;

vector<string> remove_whitespaces(const vector<string> &parts)
{
    vector<string> result;
    for (const auto &part : parts)
    {
        if (part.empty() || part[0] == ' ')
        {
            continue;
        }
        result.push_back(part);
    }
    return result;
}

vector<string> split_and_remove(const string input)
{
    vector<string> parts = split(input, ' ');
    return remove_whitespaces(parts);
}

long long get_total(const string symbols, map<int, vector<long long>> &registers)
{
    vector<string> parts = split_and_remove(symbols);
    long long total = 0;
    for (int s = 0; s < parts.size(); s++)
    {
        string symbol = parts[s];
        long long symbol_total = symbol == "*" ? 1 : 0;
        vector<long long> vals = registers[s];
        for (const auto &val : vals)
        {
            if (symbol == "+")
            {
                symbol_total += val;
            }
            else if (symbol == "*")
            {
                symbol_total *= val;
            }
        }
        total += symbol_total;
    }
    return total;
}

long long part1(vector<string> lines)
{
    map<int, vector<long long>> registers;
    for (int l = 0; l < lines.size() - 1; l++)
    {
        string line = lines[l];
        vector<string> parts = split_and_remove(line);
        for (int i = 0; i < parts.size(); i++)
        {
            registers[i].push_back(stoll(parts[i]));
        }
    }

    string symbols = lines[lines.size() - 1];
    vector<string> parts = split_and_remove(symbols);
    long long total = get_total(symbols, registers);
    return total;
}

long long part2(vector<string> lines)
{
    map<pair<int, int>, char> grid;
    for (int r = 0; r < lines.size(); r++)
    {
        for (int c = 0; c < lines[r].size(); c++)
        {
            grid[{r, c}] = lines[r][c];
        }
    }

    map<int, vector<long long>> registers;
    int col = 0;
    int n = 0;
    while (col < lines[0].size())
    {
        string number;
        for (int row = 0; row < lines.size() - 1; row++)
        {
            char c = grid[{row, col}];
            if (c == ' ')
            {
                continue;
            }
            number += c;
        }
        if (number.empty())
        {
            n++;
        }
        else
        {
            registers[n].push_back(stoll(number));
        }
        col++;
    }

    string symbols = lines[lines.size() - 1];
    vector<string> parts = split_and_remove(symbols);
    long long total = get_total(symbols, registers);
    return total;
}
