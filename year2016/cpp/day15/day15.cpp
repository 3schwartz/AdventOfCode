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


vector<pair<int, int> > parse_data() {
    const auto data = read_lines("../../data/day15_data.txt");
    vector<pair<int, int> > disc_positions;

    for (int i = 0; i < data.size(); i++) {
        const string &line = data[i];
        int positions, start;
        sscanf(line.c_str(), "Disc #%*d has %d positions; at time=0, it is at position %d.", &positions, &start);

        disc_positions.emplace_back(positions, mod(-start - (i + 1), positions));
    }

    return disc_positions;
}

/**
 * For each disc, we want to find a time t such that:
 *   (position + t + index + 1) ≡ 0 (mod positions)
 *
 * This is a congruence. Using modular arithmetic:
 *   If a ≡ b (mod m), then a - c ≡ b - c (mod m)
 *
 * Therefore, we can subtract (position + index + 1) from both sides:
 *   t ≡ - (position + index + 1) (mod positions)
 *
 * Each disc contributes one such congruence. Together, they form a system of
 * simultaneous congruences:
 *
 *   t ≡ a₁ (mod n₁)
 *   t ≡ a₂ (mod n₂)
 *   ...
 *
 * This system can be solved using the Chinese Remainder Theorem,
 * provided that all 'positions' values (the moduli nᵢ) are pairwise coprime.
 */
int find_earliest_time(vector<pair<int, int> > disc_positions) {
    std::ranges::sort(disc_positions, [](const auto &a, const auto &b) {
        return a.first > b.first;
    });
    const auto [disc, position] = disc_positions.front();
    int inc = disc;
    int earliest = position;

    for (size_t i = 1; i < disc_positions.size(); ++i) {
        const auto [disc, position] = disc_positions[i];
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
    vector<pair<int, int> > disc_positions{
        {5, mod(-4 - 1, 5)},
        {2, mod(-1 - 2, 2)},
    };

    int earliest = find_earliest_time(disc_positions);
    if (earliest != 5) {
        std::cerr << "Test failed, got: " << earliest << std::endl;
        return 1;
    }
    cout << "Test: " << earliest << endl;

    disc_positions = parse_data();
    earliest = find_earliest_time(disc_positions);
    cout << "Part 1: " << earliest << endl;

    disc_positions.emplace_back(11, mod(-(static_cast<int>(disc_positions.size() + 1)), 11));
    earliest = find_earliest_time(disc_positions);
    cout << "Part 2: " << earliest << endl;

    return 0;
}

