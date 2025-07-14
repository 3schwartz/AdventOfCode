#include "../common/common.h"
#include "day19.h"

int main() {
    constexpr unsigned input = 3018458;

    cout << "Part 1: " << steal_left(input) << endl;

    cout << "Part 2: " << steal_above(input) << endl;

    return 0;
}
