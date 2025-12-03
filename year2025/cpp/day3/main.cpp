#include "../common/common.h"
#include "aoc.h"
#include <string>

using std::min;
using std::to_string;

int main()
{
    const vector<string> lines = read_lines("../../../../data/day3_data.txt");

    int voltage = 0;

    for (const string &line : lines)
    {
        int first = 0;
        int second = 0;

        for (char c : line)
        {
            int value = static_cast<int>(c - '0');
            if (first == 0 || second == 0)
            {
                first = second;
                second = value;
                continue;
            }
            if (value > second && second < first)
            {
                second = value;
                continue;
            }
            if (value > first || value > second)
            {
                first = second;
                second = value;
                continue;
            }
            if (first < second)
            {
                first = second;
                second = value;
                continue;
            }
        }

        string combined = to_string(first) + to_string(second);
        int combined_value = std::stoi(combined);

        voltage += combined_value;
    }

    long long joltage_12 = 0;
    for (const string &line : lines)
    {
        std::vector<int> values(12, 0);
        for (size_t i = 0; i < line.size(); ++i)
        {
            char c = line[i];
            int value = static_cast<int>(c - '0');
            int j = std::min<int>(line.size() - i, 12);
            for (; j > 0; --j)
            {
                if (value <= values[12 - j])
                {
                    continue;
                }
                values[12 - j] = value;
                for (int k = j - 1; k > 0; --k)
                {
                    values[12 - k] = 0;
                }
                break;
            }
        }
        std::string key;
        key.reserve(values.size());
        for (int v : values)
        {
            key += std::to_string(v);
        }

        long long j = std::stoll(key);
        joltage_12 += j;
    }

    cout << "Part 1: " << voltage << endl;
    cout << "Part 2: " << joltage_12 << endl;

    return 0;
}
