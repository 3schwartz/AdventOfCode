#include <iostream>

#include "day17.h"

static constexpr string salt = "gdjjyniy";

using std::cout;
using std::endl;

int main() {
    const auto [path,_] = shortest_path(salt, true);

    cout << "Part 1: " << path << endl;

    const auto [__, steps] = shortest_path(salt, false);

    cout << "Part 2: " << steps << endl;
    return 0;
}
