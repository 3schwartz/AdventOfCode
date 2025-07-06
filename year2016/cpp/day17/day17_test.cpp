#include "day17.h"

#include <string>
#include <gtest/gtest.h>

TEST(DAY17, CorrectPath) {
    // Arrange
    const std::string salt = "ihgpwlah";

    // Act
    const string path = shortest_path(salt);


    // Assert
    EXPECT_EQ(path, "DDRRRD");
}
