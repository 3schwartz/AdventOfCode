#include <bit>
#include <cmath>

using std::bit_width;
using std::pow;

int steal_left(const unsigned int n) {
    const int power = bit_width(n) - 1;
    const int floor = static_cast<int>(pow(2, power));
    const int shift = n == floor ? 0 : (static_cast<int>(n) - floor) / 2 * 4;
    return 1 + shift;
}




