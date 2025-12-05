#include "aoc.h"
#include "../common/common.h"
#include <cassert>

using std::to_string;

void test_answer_42()
{
    assert(answer() == 42);
}

void test_answer_not_0()
{
    assert(answer() != 0);
}

void test_part1()
{
    vector<string> lines = read_lines("../../../../data/day5_data_test.txt");
    int result = part1(lines);
    assert(result == 3);
}

void test_part2()
{
    vector<string> lines = read_lines("../../../../data/day5_data_test.txt");
    int result = part2(lines);
    assert(result == 14);
}

int main()
{
    test_answer_42();
    test_answer_not_0();
    test_part1();
    test_part2();

    return 0;
}
