#include "day14.h"

#include <gtest/gtest.h>

TEST(DAY14, CorrectHash) {
    // Arrange
    const string salt = "abc";
    constexpr int index = 18;

    // Act
    const string hash = md5_hash(index, salt);

    // Assert
    EXPECT_NE(hash.find("cc38887a5"), std::string::npos);
}

TEST(DAY14, Correct64Key) {
    // Arrange
    const string salt = "abc";
    constexpr int key_number = 64;
    constexpr int limit = 1000;

    // Act
    const int index = find_key(key_number, salt, limit);

    // Assert
    EXPECT_EQ(index, 22728);
}
