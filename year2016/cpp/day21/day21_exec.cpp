#include <vector>
#include "../common/common.h""
#include "day21.h"

using std::vector;

string handle_events(const string &password, const vector<string> &data) {
    string pass = password;
    for (const auto &line: data) {
        pass = handle_event(pass, line);
    }
    return pass;
}

int main() {
    const auto data = read_lines("../../data/day21_data.txt");
    string password = "abcdefgh";

    cout << "Part 1: " << handle_events(password, data) << endl;
    return 0;
}
