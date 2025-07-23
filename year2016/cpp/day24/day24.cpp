#include "../common/common.h"

int main() {
    const auto data = read_lines("../../data/day24_data.txt");

    for (const auto &line: data) {
        cout << line << endl;
    }
}
