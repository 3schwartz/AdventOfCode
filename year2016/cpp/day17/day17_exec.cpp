#include <iostream>

#include "day17.h"

static constexpr string salt = "gdjjyniy";

using std::cout;
using std::endl;

int main() {
    const string path = shortest_path(salt);

    cout << "Part 1: " << path << endl;
    return 0;
}
