#include <iostream>
#include <map>
#include <sstream>

#include "../common/common.h"

using std::cout;
using std::map;
using std::string;
using std::istringstream;
using std::endl;
using std::invalid_argument;

int get_value(map<string, int> &registers, const string &value) {
    int v;
    if (istringstream iss(value); iss >> v && iss.eof()) {
        return v;
    }

    return registers[value];
}

int run(map<string, int> registers, vector<string> data) {
    size_t index = 0;
    while (index < data.size()) {
        istringstream iss(data[index]);
        string action, x, y;
        iss >> action >> x >> y;

        if (action == "cpy") {
            registers[y] = get_value(registers, x);
        } else if (action == "inc") {
            registers[x]++;
        } else if (action == "dec") {
            registers[x]--;
        } else if (action == "jnz") {
            if (get_value(registers, x) != 0) {
                index += get_value(registers, y);
                continue;
            }
        } else {
            throw invalid_argument("Invalid action: " + action);
        }
        index++;
    }

    return registers["a"];
}

int main() {
    const auto data = read_lines("../../data/day12_data.txt");
    map<string, int> registers;

    int part_1 = run(registers, data);
    cout << "Part 1: " << part_1 << endl;

    registers["c"] = 1;
    int part_2 = run(registers, data);
    cout << "Part 2: " << part_2 << endl;

    return 0;
}
