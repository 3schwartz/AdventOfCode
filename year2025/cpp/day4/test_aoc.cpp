#include "aoc.h"
#include <cassert>

void test_answer_42()
{
    assert(answer() == 42);
}

void test_answer_not_0()
{
    assert(answer() != 0);
}

int main()
{
    test_answer_42();
    test_answer_not_0();

    return 0;
}
