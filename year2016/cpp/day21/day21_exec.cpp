#include <vector>
#include "../common/common.h"
#include "day21.h"

using std::vector;

string handle_events(const string &password, const vector<string> &data, const bool reverse = false) {
    string pass = password;
    vector<string> events = data;
    if (reverse) {
        std::ranges::reverse(events);
    }
    for (const auto &line: events) {
        pass = handle_event(pass, line, reverse);
    }
    return pass;
}

int main() {
    const auto data = read_lines("../../data/day21_data.txt");

    cout << "Part 1: " << handle_events("abcdefgh", data) << endl;
    cout << "Part 2: " << handle_events("fbgdceah", data, true) << endl;

    return 0;
}
