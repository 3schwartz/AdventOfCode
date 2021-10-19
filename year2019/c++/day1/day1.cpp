#include <iostream>
#include <fstream>
#include <set>

using namespace std;

int get_fuel(int mass) {
    return floor((mass / 3) - 2);
}

int main()
{
    ifstream infile;
    infile.open("day1_data.txt");

    set<int> mp;
    int val;
    while (infile >> val) {
        mp.insert(val);
    }

    int sum = 0;

    for (auto it = mp.begin(); it != mp.end(); ++it) {
        sum += get_fuel(*it);
    }

    cout << "Part 1: " << sum << endl;

    int sum_part_2 = 0;

    for (auto it = mp.begin(); it != mp.end(); ++it) {
        
        int last_value = *it;

        while (true) {
            last_value = get_fuel(last_value);

            if (last_value <= 0) {
                break;
            }

            sum_part_2 += last_value;
        }
    }

    cout << "Part 2: " << sum_part_2 << endl;
}
