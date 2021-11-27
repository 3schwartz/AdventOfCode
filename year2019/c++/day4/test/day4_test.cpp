#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../doctest.h"

using namespace std;

class PasswordFinder
{
    private:
        int from;
        int to;

    public:
        PasswordFinder(int from, int to) : from(from), to(to) {}
        std::vector<int> getPasswords(){
            std::vector<int> passwords;
            for(int i = from; i <= to; i++) {
                passwords.push_back(i);
            }
            return passwords;
        };
};

class Validation
{
    public:
        virtual bool Validate(int password) = 0;
};

class IncreasingValidation : public Validation
{
    public:
        bool Validate(int password) {
            
            string passwordString = to_string(password);

            bool less = false;
            for(int i = 0; i < passwordString.size() - 1; i++) {
                less = passwordString[i] <= passwordString[i+1];
                if (less == false) {
                    break;
                }
            }
            return less;
        };
};

class TwoSequentuallyEqual : public Validation
{
    public:
        bool Validate(int password) {
            
            string passwordString = to_string(password);

            bool equal = false;
            for(int i = 0; i < passwordString.size() - 1; i++) {
                equal = passwordString[i] == passwordString[i+1];
                if (equal) {
                    break;
                }
            }

            return equal;
        };
};

TEST_CASE("Two sequentially equal") {
    SUBCASE("When one place two sequentially equal then true"){
        // Arrange
        int password = 1123;

        // Act
        Validation* twoSequentuallyEqual = new TwoSequentuallyEqual;
        bool validationTwoSequentuallyEqual = twoSequentuallyEqual->Validate(password);

        // Arrange
        CHECK(validationTwoSequentuallyEqual);
        delete twoSequentuallyEqual;
    }

    SUBCASE("When no sequantially equal then false"){
        // Arrange
        int password = 1234;

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
        int passwordInt = 1213;

        // Act
        IncreasingValidation increaseValidation;
        bool less = increaseValidation.Validate(passwordInt);
        
        // Assert
        CHECK(!less);
    }

    SUBCASE("Decreasing number should result in false") {
        // Arrange
        int passwordInt = 4321;

        // Act
        IncreasingValidation increaseValidation;
        bool less = increaseValidation.Validate(passwordInt);
        
        // Assert
        CHECK(!less);
    }

    SUBCASE("Increasing number should result in true") {
        // Arrange
        int passwordInt = 1234;

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