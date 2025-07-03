#include <iostream>
#include <map>
#include "../common/common.h"

using std::cout;
using std::endl;
using std::map;
using std::vector;
using std::pair;
using std::sscanf;
using std::size_t;


map<int, int> parse_data() {
    const auto data = read_lines("../../data/day15_data.txt");
    map<int, int> disc_positions;

    for (int i = 0; i < data.size(); i++) {
        const string &line = data[i];
        int positions, start;
        sscanf(line.c_str(), "Disc #%*d has %d positions; at time=0, it is at position %d.", &positions, &start);

        disc_positions.insert({positions, mod(-start - (i + 1), positions)});
    }

    return disc_positions;
}

int find_earliest_time(const map<int, int> &disc_positions) {
    const auto last = std::prev(disc_positions.end());
    int inc = last->first;
    int earliest = last->second;

    for (auto [disc, position]: disc_positions) {
        while (true) {
            const int earliest_mod_disc = mod(earliest, disc);
            if (earliest_mod_disc == position) {
                break;
            }
            earliest += inc;
        };
        inc *= disc;
    }
    return earliest;
}

// Disc #1 has 5 positions; at time=0, it is at position 4.
// Disc #2 has 2 positions; at time=0, it is at position 1.

int main() {
    map<int, int> disc_positions{
        {5, mod(-4 - 1, 5)},
        {2, mod(-1 - 2, 2)},
    };
    for (auto [disc, position]: disc_positions) {
        cout << disc << " " << position << endl;
    }

    int earliest = find_earliest_time(disc_positions);
    cout << "Test: " << earliest << endl;

    disc_positions = parse_data();
    earliest = find_earliest_time(disc_positions);
    cout << "Part 1: " << earliest << endl;

    disc_positions.insert({11, mod(-(static_cast<int>(disc_positions.size() + 1)), 11)});
    earliest = find_earliest_time(disc_positions);
    cout << "Part 2: " << earliest << endl;

    return 0;
}
