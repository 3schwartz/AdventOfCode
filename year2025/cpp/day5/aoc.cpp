#include "aoc.h"
#include "../common/common.h"
#include <set>

using std::pair;
using std::set;
using std::stoll;
using std::vector;

int answer()
{
    return 42;
}

struct Parsed
{
    vector<pair<long long, long long>> ranges;
    vector<long long> fresh_ids;
};

Parsed parse_input(const vector<string> &lines)
{

    vector<long long> fresh_ids;
    vector<pair<long long, long long>> ranges;
    for (const string &line : lines)
    {
        vector<string> parts = split(line, '-');
        if (parts.size() == 2)
        {
            long long a = stoll(parts[0]);
            long long b = stoll(parts[1]);
            ranges.push_back({a, b});
        }
        if (parts.size() == 1)
        {
            auto id = stoll(parts[0]);
            fresh_ids.push_back(id);
        }
    }
    return Parsed{ranges, fresh_ids};
}

int part1(vector<string> lines)
{
    Parsed parsed = parse_input(lines);

    int valid = 0;

    for (long long id : parsed.fresh_ids)
    {
        for (const auto &range : parsed.ranges)
        {
            if (id >= range.first && id <= range.second)
            {
                valid++;
                break;
            }
        }
    }

    return valid;
}

long long part2(vector<string> lines)
{
    Parsed parsed = parse_input(lines);

    sort(parsed.ranges.begin(), parsed.ranges.end());
    vector<pair<long long, long long>> combined;
    for (const auto &interval : parsed.ranges)
    {
        if (!combined.empty() && interval.first <= combined.back().second)
        {
            combined.back().second = std::max(interval.second, combined.back().second);
        }
        else
        {
            combined.push_back(interval);
        }
    }
    long long fresh = 0;
    for (auto &p : combined)
    {
        fresh += (p.second - p.first + 1);
    }

    return fresh;
}
