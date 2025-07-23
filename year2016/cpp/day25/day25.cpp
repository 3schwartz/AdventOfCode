#include "../common/common.h"

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

vector<int> run(map<string, int> registers, vector<string> data) {
    size_t index = 0;
    vector<int> outs;
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
        } else if (action == "out") {
            outs.push_back(get_value(registers, x));
            if (outs.size() == 20) {
                break;
            }
        } else {
            throw invalid_argument("Invalid action: " + action);
        }
        index++;
    }

    return outs;
}

int main() {
    const auto data = read_lines("../../data/day25_data.txt");

    vector<int> expected;
    expected.reserve(20);
    for (int i = 0; i < 10; ++i) {
        expected.push_back(0);
        expected.push_back(1);
    }

    int a = 0;
    while (true) {
        map<string, int> registers;
        registers["a"] = a;
        vector<int> outs = run(registers, data);
        if (outs == expected) {
            cout << "Part 1: " << a << endl;
            break;
        }
        a++;
    }
}
