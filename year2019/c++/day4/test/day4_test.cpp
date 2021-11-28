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
        virtual bool Validate(std::string password) = 0;
};

class IncreasingValidation : public Validation
{
    public:
        bool Validate(std::string password) {

            bool less = false;
            for(int i = 0; i < password.size() - 1; i++) {
                less = password[i] <= password[i+1];
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
        bool Validate(std::string password) {

            bool equal = false;
            for(int i = 0; i < password.size() - 1; i++) {
                equal = password[i] == password[i+1];
                if (equal) {
                    break;
                }
            }

            return equal;
        };
};

class PasswordValidator
{
    private:
        std::vector<Validation*> validations;
    
    public:
        ~PasswordValidator()
        {
            this->validations.clear();
        }

        void setPasswordValidations(std::vector<Validation*> validations) {
            this->validations = validations;
        }

        bool validatePassword(int password){

            string passwordString = to_string(password);

            bool validPassword = false;
            for(int i = 0; i < validations.size(); i++) {
                validPassword = validations[i]->Validate(passwordString);
                if(!validPassword) {
                    break;
                }
            }

            return validPassword;
        };

        int numberValidPassword(std::vector<int> passwords) {
            int numberValid = 0;

            std::for_each(std::begin(passwords), std::end(passwords),
             [this, &numberValid](int password){
                 numberValid += this->validatePassword(password) ? 1 : 0;
             });

             return numberValid;
        }
};

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