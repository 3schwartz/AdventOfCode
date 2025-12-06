#include "aoc.h"
#include "../common/common.h"
#include <cassert>

void test_part1()
{
    vector<string> lines = read_lines("../../../../data/day6_data_test.txt");
    long long result = part1(lines);
    assert(result == 4277556);
}

void test_part2()
{
    vector<string> lines = read_lines("../../../../data/day6_data_test.txt");
    long long result = part2(lines);
    assert(result == 3263827);
}

int main()
{
    test_part1();
    test_part2();

    return 0;
}
