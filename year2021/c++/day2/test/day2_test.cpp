#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../../../external/doctest/doctest.h"
#include "../include/day2_func.h"


TEST_CASE("Test correct move") {
    // Arrange
    std::string inputString = "forward 5";

    // Act
    Move move = Move(inputString);

    // Assert
    CHECK("forward" == move.direction);
    CHECK(5 == move.step);

}