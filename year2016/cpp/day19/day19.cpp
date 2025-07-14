#include <bit>
#include <cmath>
#include <iostream>
#include <numeric>
#include <vector>
#include <__ostream/basic_ostream.h>

using std::bit_width;
using std::pow;
using std::vector;

/**
 * Given an even number (only validated even), the number series goes as follows:
 *
 * When a number is divisible by 2^X (a power of 2), the series restarts and
 * the result is 1.
 * Otherwise, it increases by 4 for each even number in the series (or 2 for each number)
 * from the last floor that was divisible by 2^X.
 */

int steal_left(const unsigned int n) {
    const int power = bit_width(n) - 1;
    const int floor = static_cast<int>(pow(2, power));
    const int shift = n == floor ? 0 : (static_cast<int>(n) - floor) / 2 * 4;
    return 1 + shift;
}


int steal_above(const unsigned int n) {
    vector<int> elems(n);
    std::iota(elems.begin(), elems.end(), 1);
    int idx = 0;
    while (true) {
        const int size = static_cast<int>(elems.size());
        if (size % 1000000 == 0) {
            std::cout << size << std::endl;
        }
        if (elems.size() == 1) {
            return elems[0];
        }
        const int idx_to_delete = (idx + size / 2) % size;
        elems.erase(elems.begin() + idx_to_delete);
        // if was last elem, then restart
        if (idx == elems.size()) {
            idx = 0;
        }
        // if above idx, then inc
        else if (idx < idx_to_delete) {
            idx = (1 + idx) % size;
        }
        // if below idx, then keep, idx = idx
    }
}
