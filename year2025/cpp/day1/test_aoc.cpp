#include "aoc.h"
#include <cassert>
#include "../common/common.h"

void test_mod()
{
    int m = -204;
    m += 100;
    m %= 100;
    assert(m == -4);
}

void test_mod_common()
{
    int m = -204;
    m = mod(m, 100);
    assert(m == 96);
}

int main()
{
    test_mod();
    test_mod_common();

    return 0;
}
