#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../../../external/doctest/doctest.h"
#include "../include/day4_func.h"

using namespace std;

TEST_CASE("Two equal") {
    SUBCASE("When two equal then true"){
        // Arrange
        std::string password = "1123";

        // Act
        Validation* twoEqual = new TwoEqual;
        bool validationTwoEqual = twoEqual->Validate(password);

        // Arrange
        CHECK(validationTwoEqual);
        delete twoEqual;
    }

    SUBCASE("When no equal then false"){
        // Arrange
        std::string password = "1234";

        // Act
        Validation* twoEqual = new TwoEqual;
        bool validationTwoEqual = twoEqual->Validate(password);

        // Arrange
        CHECK(validationTwoEqual == false);
        delete twoEqual;
    }
}

TEST_CASE("Password Validator") {
    SUBCASE ("When given range then return correct count") {
        // Arrange
        std::vector<int> passwords{12344, 12345};

        PasswordValidator* validator = new PasswordValidator;
        std::vector<Validation*> validations;

        validations.push_back(new TwoSequentuallyEqual);
        validations.push_back(new IncreasingValidation);

        validator->setPasswordValidations(validations);

        int numberValidPassword = validator->numberValidPassword(passwords);

        CHECK(numberValidPassword == 1);
    }

    SUBCASE ("Wrong password validated to false") {
        // Arrange
        int password = 1234;
        PasswordValidator* validator = new PasswordValidator;
        std::vector<Validation*> validations;

        validations.push_back(new TwoSequentuallyEqual);
        validations.push_back(new IncreasingValidation);

        validator->setPasswordValidations(validations);

        // Act
        bool passwordValid = validator->validatePassword(password);

        // Assert
        CHECK(passwordValid == false);
    }

    SUBCASE("Correct password validated to true") {
        // Arrange
        int password = 1233;
        PasswordValidator* validator = new PasswordValidator;
        std::vector<Validation*> validations;

        validations.push_back(new TwoSequentuallyEqual);
        validations.push_back(new IncreasingValidation);

        validator->setPasswordValidations(validations);

        // Act
        bool passwordValid = validator->validatePassword(password);

        // Assert
        CHECK(passwordValid);
    }
}


TEST_CASE("Two sequentially equal") {
    SUBCASE("When one place two sequentially equal then true"){
        // Arrange
        std::string password = "1123";

        // Act
        Validation* twoSequentuallyEqual = new TwoSequentuallyEqual;
        bool validationTwoSequentuallyEqual = twoSequentuallyEqual->Validate(password);

        // Arrange
        CHECK(validationTwoSequentuallyEqual);
        delete twoSequentuallyEqual;
    }

    SUBCASE("When no sequantially equal then false"){
        // Arrange
        std::string password = "1234";

        // Act
        Validation* twoSequentuallyEqual = new TwoSequentuallyEqual;
        bool validationTwoSequentuallyEqual = twoSequentuallyEqual->Validate(password);

        // Arrange
        CHECK(!validationTwoSequentuallyEqual);
        delete twoSequentuallyEqual;
    }
}

TEST_CASE("Increase rule ") {
    SUBCASE("Decreasing number in middle should result in false") {
        // Arrange
        std::string passwordInt = "1213";

        // Act
        IncreasingValidation increaseValidation;
        bool less = increaseValidation.Validate(passwordInt);
        
        // Assert
        CHECK(!less);
    }

    SUBCASE("Decreasing number should result in false") {
        // Arrange
        std::string passwordInt = "4321";

        // Act
        IncreasingValidation increaseValidation;
        bool less = increaseValidation.Validate(passwordInt);
        
        // Assert
        CHECK(!less);
    }

    SUBCASE("Increasing number should result in true") {
        // Arrange
        std::string passwordInt = "1234";

        // Act
        IncreasingValidation increaseValidation;
        bool less = increaseValidation.Validate(passwordInt);
        
        // Assert
        CHECK(less);
    }
}

TEST_CASE("Correct range") {
    // Arrange
    int from = 1;
    int to = 3;
    PasswordFinder passwordFinder(from, to);

    // Act
    std::vector<int> passwords = passwordFinder.getPasswords();

    // Assert
    CHECK(passwords.size() == 3);
    CHECK(passwords[0] == 1);
    CHECK(passwords[1] == 2);
    CHECK(passwords[2] == 3);
}