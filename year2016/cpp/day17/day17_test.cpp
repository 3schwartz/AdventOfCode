#include "day17.h"

#include <string>
#include <gtest/gtest.h>

TEST(DAY17, CorrectPathPart2) {
    // Arrange
    const std::string salt = "ihgpwlah";

    // Act
    const auto [_,steps] = shortest_path(salt, false);


    // Assert
    EXPECT_EQ(steps, 370);
}

TEST(DAY17, CorrectPath) {
    // Arrange
    const std::string salt = "ihgpwlah";

    // Act
    const auto [path,_] = shortest_path(salt, true);


    // Assert
    EXPECT_EQ(path, "DDRRRD");
}
