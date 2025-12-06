#include "aoc.h"
#include "../common/common.h"
#include <cassert>

void test_part1()
{
    vector<string> lines = read_lines("../../../../data/day7_data_test.txt");
    int result = part1(lines);
    assert(result == -1);
}

void test_part2()
{
    vector<string> lines = read_lines("../../../../data/day7_data_test.txt");
    int result = part2(lines);
    assert(result == -1);
}

int main()
{
    test_part1();
    test_part2();

    return 0;
}
