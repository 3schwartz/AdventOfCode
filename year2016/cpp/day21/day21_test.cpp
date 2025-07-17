#include "day21.h"
#include <gtest/gtest.h>

using std::string;

TEST(DAY21, SwapPositions) {
    // Arrange
    string password = "abcde";

    // Act
    password = handle_swap_position(password, "swap position 4 with position 0");

    // Assert
    EXPECT_EQ(password, "ebcda");
}

TEST(DAY21, SwapPositionsReverse) {
    // Arrange
    string password = "ebcda";

    // Act
    password = handle_swap_position_reverse(password, "swap position 4 with position 0");

    // Assert
    EXPECT_EQ(password, "abcde");
}

TEST(DAY21, SwapLetter) {
    // Arrange
    string password = "ebcda";

    // Act
    password = handle_swap_letter(password, "swap letter d with letter b");

    // Assert
    EXPECT_EQ(password, "edcba");
}

TEST(DAY21, SwapLetterReverse) {
    // Arrange
    string password = "edcba";

    // Act
    password = handle_swap_letter_reverse(password, "swap letter d with letter b");

    // Assert
    EXPECT_EQ(password, "ebcda");
}

TEST(DAY21, RotateBasedOnPositionOfLetter1) {
    // Arrange
    string password = "abdec";

    // Act
    password = handle_rotate_based_on_position_of_letter(password, "rotate based on position of letter b");

    // Assert
    EXPECT_EQ(password, "ecabd");
}

TEST(DAY21, RotateBasedOnPositionOfLetter1Reverse) {
    // Arrange
    string password = "ecabd";

    // Act
    password = handle_rotate_based_on_position_of_letter_reverse(password, "rotate based on position of letter b",
                                                                 true);

    // Assert
    EXPECT_EQ(password, "abdec");
}

TEST(DAY21, RotateBasedOnPositionOfLetter2) {
    // Arrange
    string password = "ecabd";

    // Act
    password = handle_rotate_based_on_position_of_letter(password, "rotate based on position of letter d");

    // Assert
    EXPECT_EQ(password, "decab");
}

TEST(DAY21, RotateBasedOnPositionOfLetter2Reverse) {
    // Arrange
    string password = "decab";

    // Act
    password = handle_rotate_based_on_position_of_letter_reverse(password, "rotate based on position of letter d",
                                                                 true);

    // Assert
    EXPECT_EQ(password, "ecabd");
}

TEST(DAY21, RotateLeft) {
    // Arrange
    string password = "abcde";

    // Act
    password = handle_rotate_left_or_right(password, "rotate left 1 step");

    // Assert
    EXPECT_EQ(password, "bcdea");
}

TEST(DAY21, RotateLeftReverse) {
    // Arrange
    string password = "bcdea";

    // Act
    password = handle_rotate_left_or_right_reverse(password, "rotate left 1 step");

    // Assert
    EXPECT_EQ(password, "abcde");
}

TEST(DAY21, Reverse) {
    // Arrange
    string password = "edcba";

    // Act
    password = handle_reverse_positions(password, "reverse positions 0 through 4");

    // Assert
    EXPECT_EQ(password, "abcde");
}

TEST(DAY21, ReverseReverse) {
    // Arrange
    string password = "abcde";

    // Act
    password = handle_reverse_positions_reverse(password, "reverse positions 0 through 4");

    // Assert
    EXPECT_EQ(password, "edcba");
}

TEST(DAY21, Move1) {
    // Arrange
    string password = "bcdea";

    // Act
    password = handle_move_position(password, "move position 1 to position 4");

    // Assert
    EXPECT_EQ(password, "bdeac");
}

TEST(DAY21, Move1Reverse) {
    // Arrange
    string password = "bdeac";

    // Act
    password = handle_move_position_reverse(password, "move position 1 to position 4");

    // Assert
    EXPECT_EQ(password, "bcdea");
}

TEST(DAY21, Move2) {
    // Arrange
    string password = "bdeac";

    // Act
    password = handle_move_position(password, "move position 3 to position 0");

    // Assert
    EXPECT_EQ(password, "abdec");
}

TEST(DAY21, Move2Reverse) {
    // Arrange
    string password = "abdec";

    // Act
    password = handle_move_position_reverse(password, "move position 3 to position 0");

    // Assert
    EXPECT_EQ(password, "bdeac");
}
