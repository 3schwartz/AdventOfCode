#include "../common/common.h"
#include "aoc.h"

using std::stoi;
using std::stol;

int main()
{
    const vector<string> lines = read_lines("../../../../data/day2_data.txt");
    string line = lines[0];
    vector<string> parts = split(line, ',');

    long ids = 0;
    long long invalid_ids = 0;

    for (const string &part : parts)
    {
        vector<string> subparts = split(part, '-');
        if (subparts.size() != 2)
        {
            std::cout << "Invalid range: " << part << "\n";
            continue;
        }
        long first = stol(subparts[0]);
        long second = stol(subparts[1]);

        for (long i = first; i <= second; ++i)
        {
            string i_s = std::to_string(i);
            if (i_s.length() % 2 != 0)
            {
                continue;
            }
            if (i_s.substr(0, i_s.length() / 2) == i_s.substr(i_s.length() / 2))
            {
                ids += i;
            }
        }

        for (long i = first; i <= second; ++i)
        {
            string i_s = std::to_string(i);

            bool invalid = false;
            for (size_t a = 1; a <= i_s.length() / 2; ++a)
            {
                if (i_s.length() % a != 0)
                {
                    continue;
                }

                bool all_same = true;
                string sub = i_s.substr(0, a);

                for (size_t pos = a; pos < i_s.length(); pos += a)
                {
                    string sub2 = i_s.substr(pos, a);
                    if (sub != sub2)
                    {
                        all_same = false;
                        break;
                    }
                }

                if (all_same)
                {
                    invalid = true;
                    break;
                }
            }
            if (invalid)
            {
                invalid_ids += i;
            }
        }
    }
    cout << "Part 1: " << ids << endl;
    cout << "Part 2: " << invalid_ids << endl;

    return 0;
}
