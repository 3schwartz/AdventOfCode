#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../../../external/doctest/doctest.h"
#include "../include/day3_func.h"

TEST_CASE("Correct wire init") {
    // Arrange
    std::vector<std::string> moves = {"R75", "D30"};

    // Act
    Wire wire(moves);

    // Assert
    CHECK(2 == wire.moves.size());
}

TEST_CASE("Move init correct") {
    // Arrange
    std::string direction = "R75";

    // Act
    Move move(direction);

    // Assert
    CHECK("R" == move.direction);
    CHECK(75 == move.steps);
}

TEST_CASE("Correct distance min") {
    // Arrange
    std::vector<std::string> wireOne = {"R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"};
    std::vector<std::string> wireTwo = {"U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"};
    
    int expected = 159;
    
    // Act
    int actual = getMinimumDistance(wireOne, wireTwo);
    
    // Assert
    CHECK(actual == expected);
}

TEST_CASE("Correct steps and dimension") {
        // Arrange
    std::vector<std::string> wireOne = {"R8", "U5", "L5", "D3"};
    std::vector<std::string> wireTwo = {"U7", "R6", "D4", "L4"};

    SUBCASE("Correct distance min second") {   
        int expected = 6;

        // Act
        int actual = getMinimumDistance(wireOne, wireTwo);
    
        // Assert
        CHECK(actual == expected);
    }

    SUBCASE("Correct distance min second") {
        int expected = 30;

        // Act
        int actual = getMinimumSteps(wireOne, wireTwo);
    
        // Assert
        CHECK(actual == expected);
    }            
}

