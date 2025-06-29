#include <iostream>
#include <string>
#include "day14.h"


using std::cout;
using std::endl;

constexpr string salt = "cuanljph";

int main() {
    int index = find_key(64, salt, 1000);

    cout << "Part 1: " << index << endl;

    index = find_key(64, salt, 1000, 2016);

    cout << "Part 2: " << index << endl;
}
