#include <iostream>

using std::cout;
using std::endl;

/*
 * The first instructions down to tgl consist of four loops.
 *
 * The first two loops increase a.
 * d holds the current state of a.
 *
 * The first jnz increases by a - x, where x is the loop count.
 * The second jnz repeats the above process d times, which ends up giving, for example, in the first round:
 * a * (a - 1)
 *
* The third loop adjusts the tgl pointer so that,
 * by the end, every second instruction after tgl is modified.
 *
 * Execution then jumps back before the first loop, controlled by b,
 * which is decreased every time the fourth loop is executed.
 * Since b is assigned a - 1 in the second instruction, the three loops end up giving
 * the factorial of a:
 * a * (a - 1) * (a - 2) * (a - 3) * ... * 1
 *
 * The fourth loop 'breaks' when b = 1, because at that point b → c → d,
 * and when d-- and c++ occur, tgl changes "jnz 1 c" into "cpy 1 c".
 *
 * At this point, tgl has modified every second instruction, creating two small loops.
 * The first increases a by 74 (controlled by d), and the second repeats this 87 times
 * (controlled by c).
 *
 * Hence, we end up with:
 * a! + 87 * 74
 */
unsigned long long short_circuit(const unsigned long long a) {
    unsigned long long result = 1;
    for (unsigned long long i = 2; i <= a; i++) {
        result *= i;
    }
    return result + 87 * 74;
}

int main() {
    const auto part_1 = short_circuit(7);
    cout << "Part 1: " << part_1 << endl;

    const auto part_2 = short_circuit(12);
    cout << "Part 2: " << part_2 << endl;

    return 0;
}
