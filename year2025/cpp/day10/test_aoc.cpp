#include "aoc.h"
#include "../common/common.h"
#include <cassert>

void test_part1(const vector<string> &lines)
{
    long long result = part1(lines);
    assert(result == 7);
}

void test_part2(const vector<string> &lines)
{
    long long result = part2(lines);
    assert(result == 33);
}

void test_machineFewestSteps()
{
    Machine machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    long long steps = machine.findFewestSteps();
    assert(steps == 2);
}

void test_machineFewestPresses()
{
    Machine machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    long long presses = machine.findFewestPresses();
    assert(presses == 10);
}

void test_solveJoltage()
{
    Machine machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    long long presses = machine.solveJoltage();
    assert(presses == 10);
}

int main()
{
    test_solveJoltage();
    test_machineFewestSteps();
    test_machineFewestPresses();
    const vector<string> lines = read_lines("../../../../data/day10_data_test.txt");
    test_part1(lines);
    test_part2(lines);

    return 0;
}
