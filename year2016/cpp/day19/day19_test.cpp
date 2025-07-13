#include "day19.h"
#include <gtest/gtest.h>

TEST(DAY19, StealLeft) {
    // Arrange
    const std::vector<std::pair<int, int> > params = {{6, 5}};

    for (const auto &[input, expected]: params) {
        // Act
        const int actual = steal_left(input);

        // Assert
        EXPECT_EQ(actual, expected);
    }
}
