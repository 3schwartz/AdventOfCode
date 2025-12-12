#include "../common/common.h"
#include "aoc.h"

int main()
{
    const vector<string> lines = read_lines("../../../../data/day10_data.txt");

    long long result_part1 = part1(lines);
    cout << "Part 1: " << result_part1 << "\n";

    long long result_part2 = part2(lines);
    cout << "Part 2: " << result_part2 << "\n";

    return 0;
}
