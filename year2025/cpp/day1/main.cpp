#include "../common/common.h"
#include "aoc.h"

int main()
{
    const vector<string> lines = read_lines("../../../../data/day1_data.txt");

    int start = 50;
    int zero_count = 0;
    int click_count = 0;

    for (const string &line : lines)
    {
        int r = std::stoi(line.substr(1));
        int position = start;

        switch (line[0])
        {
        case 'L':
            position -= r;
            break;
        case 'R':
            position += r;
            break;
        default:
            throw std::invalid_argument("Invalid direction: " + line);
        }

        if (position <= 0 || 99 < position)
        {
            click_count += std::abs(position) / 100 + (position <= 0 ? 1 : 0);
        }
        if (position < 0 && start == 0)
        {
            click_count--;
        }

        start = mod(position, 100);
        if (start == 0)
        {
            zero_count++;
        }
    }

    cout << "Part 1: " << zero_count << "\n";
    cout << "Part 2: " << click_count << "\n";

    return 0;
}
