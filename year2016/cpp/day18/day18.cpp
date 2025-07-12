#include <set>
#include "../common/common.h"

using std::set;
using std::pair;
using std::make_pair;

bool is_trap(set<pair<int, int> > &traps, int col, int row) {
    const pair<int, int> left = make_pair(col - 1, row);
    const pair<int, int> center = make_pair(col, row);
    const pair<int, int> right = make_pair(col + 1, row);
    if (
        traps.contains(left) &&
        traps.contains(center) &&
        !traps.contains(right)) {
        return true;
    }
    if (
        !traps.contains(left) &&
        traps.contains(center) &&
        traps.contains(right)) {
        return true;
    }
    if (
        traps.contains(left) &&
        !traps.contains(center) &&
        !traps.contains(right)) {
        return true;
    }
    if (
        !traps.contains(left) &&
        !traps.contains(center) &&
        traps.contains(right)) {
        return true;
    }
    return false;
}

int find_safe_tiles(const string &line, const int size) {
    set<pair<int, int> > traps;

    for (int i = 0; i < line.size(); i++) {
        const char c = line[i];
        if (c == '^') {
            traps.insert(pair(i, 0));
        }
    }

    for (int row = 0; row < size - 1; row++) {
        string new_line(line.size(), '.');
        for (int col = 0; col < line.size(); col++) {
            if (is_trap(traps, col, row)) {
                traps.emplace(col, row + 1);
                new_line[col] = '^';
            }
        }
    }

    return size *
           static_cast<int>(line.size()) -
           static_cast<int>(traps.size());
}

int main() {
    const auto data = read_lines("../../data/day18_data.txt");
    const string &line = data[0];

    cout << "Part 1: " << find_safe_tiles(line, 40) << endl;
    cout << "Part 2: " << find_safe_tiles(line, 400000) << endl;
    return 0;
}
