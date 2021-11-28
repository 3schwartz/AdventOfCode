#include "../include/day4_func.h"
#include <iostream>

using namespace std;

int main() {
    PasswordFinder passwordFinder(272091, 815432);
    std::vector<int> passwords = passwordFinder.getPasswords();

    PasswordValidator validator;
    std::vector<Validation*> validations;

    validations.push_back(new TwoSequentuallyEqual);
    validations.push_back(new IncreasingValidation);
    
    validator.setPasswordValidations(validations);

    int numberValidPassword = validator.numberValidPassword(passwords);

    cout << "Part 1: " << numberValidPassword << endl;

    validations.push_back(new TwoEqual);
    validator.setPasswordValidations(validations);

    int numberValidPasswordWithGroup = validator.numberValidPassword(passwords);

    cout << "Part 2: " << numberValidPasswordWithGroup << endl;
}