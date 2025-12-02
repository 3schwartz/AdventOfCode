#include "../common/common.h"
#include "aoc.h"

int main()
{
    const vector<string> lines = read_lines("../../../../data/day3_data.txt");

    for (const string &line : lines)
    {
        cout << line << endl;
    }

    cout << "Answer: " << answer() << endl;

    return 0;
}
