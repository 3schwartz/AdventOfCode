#include "aoc.h"
#include "../common/common.h"
#include <cassert>

void test_part1()
{
    vector<string> lines = read_lines("../../../../data/day9_data_test.txt");
    long long result = part1(lines);
    assert(result == -1);
}

void test_part2()
{
    vector<string> lines = read_lines("../../../../data/day9_data_test.txt");
    long long result = part2(lines);
    assert(result == -1);
}

int main()
{
    test_part1();
    test_part2();

    return 0;
}
