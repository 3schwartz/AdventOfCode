#include "../common/common.h"
#include "aoc.h"

int main()
{
    const vector<string> lines = read_lines("../../../../data/day1_data.txt");

    int start = 50;
    int zero_count = 0;
    int click_count = 0;

    int loop_count = 0;
    int loop_start = 50;
    for (const string &line : lines)
    {
        int r = std::stoi(line.substr(1));
        int start_after = start;

        switch (line[0])
        {
        case 'L':
            start_after -= r;
            break;
        case 'R':
            start_after += r;
            break;
        default:
            throw std::invalid_argument("Invalid direction: " + line);
        }
        for (int i = 1; i <= r; i++)
        {
            if (line[0] == 'L')
            {
                loop_start = (loop_start - 1 + 100) % 100;
            }
            else
            {
                loop_start = (loop_start + 1) % 100;
            }
            if (loop_start == 0)
            {
                loop_count++;
            }
        }

        int start_before = start;
        start = mod(start_after, 100);
        if (start == 0)
        {
            zero_count++;
        }
        if (start_after > 99)
        {
            click_count += start_after / 100;
        }
        if (start_after < 0)
        {
            click_count += (std::abs(start_after)) / 100 + 1;

            if (start_before == 0)
            {
                click_count--;
            }
        }
        if (start_after == 0)
        {
            click_count++;
        }
    }

    cout << "Part 1: " << zero_count << "\n";
    cout << "Part 2: " << click_count << "\n";
    cout << "Part 2: " << loop_count << "\n";

    return 0;
}
