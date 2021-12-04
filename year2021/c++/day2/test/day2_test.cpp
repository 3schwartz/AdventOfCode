#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../../../external/doctest/doctest.h"
#include "../include/day2_func.h"

using namespace std;

TEST_CASE("Test correct aim") {
    // Arrange
    vector<Move> moves = vector<Move>{Move("forward 5"),
        Move("down 5"),
        Move("forward 8"),
        Move("up 3")};
    AimCalculator calculator;

    // Act
    int position = calculator.GetPosition(moves);

    // Assert
    CHECK(520 == position);
}

TEST_CASE("Test correct position") {
    // Arrange
    vector<Move> moves = vector<Move>{Move("forward 5"),
        Move("down 5"),
        Move("forward 8"),
        Move("up 3")};
    
    RouteCalculator calculator;

    // Act
    int position = calculator.GetPosition(moves);

    // Assert
    CHECK(26 == position);
}

TEST_CASE("Test correct move") {
    // Arrange
    std::string inputString = "forward 5";

    // Act
    Move move(inputString);

    // Assert
    CHECK("forward" == move.direction);
    CHECK(5 == move.step);

}