#include "day19.h"
#include <gtest/gtest.h>

TEST(DAY19, StealLeft) {
    // Arrange
    const std::vector<std::pair<int, int> > params = {
        {6, 5},
        {10, 5},
        {12, 9},
        {14, 13},
        {26, 21},
    };

    for (const auto &[input, expected]: params) {
        // Act
        const int actual = steal_left(input);

        // Assert
        EXPECT_EQ(actual, expected);
    }
}

TEST(DAY19, StealAbove) {
    // Arrange
    const std::vector<std::pair<int, int> > params = {
        {5, 2},
        {6, 3},
        {9, 9},
        {11, 2},
        {15, 6},
        {16, 7},
    };

    for (const auto &[input, expected]: params) {
        // Act
        const int actual = steal_above(input);

        // Assert
        EXPECT_EQ(actual, expected);
    }
}
