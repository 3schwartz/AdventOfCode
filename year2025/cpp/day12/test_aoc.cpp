#include "aoc.h"
#include "../common/common.h"
#include <cassert>

void test_part1(const vector<string> &lines)
{
    long long result = part1(lines);
    assert(result == 2);
}

void test_part2(const vector<string> &lines)
{
    long long result = part2(lines);
    assert(result == -1);
}

int main()
{
    const vector<string> lines = read_lines("../../../../data/day12_data_test.txt");
    test_part1(lines);
    test_part2(lines);

    return 0;
}
