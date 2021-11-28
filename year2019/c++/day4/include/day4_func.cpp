#include "day4_func.h"
#include "../../../../external/cppitertools/itertools.hpp"

PasswordFinder::PasswordFinder(int from, int to) : from(from), to(to) {}

std::vector<int> PasswordFinder::getPasswords() {
    std::vector<int> passwords;
    for(int i = from; i <= to; i++) {
        passwords.push_back(i);
    }
    return passwords;
};

bool IncreasingValidation::Validate(std::string password) {
    bool less = false;
    for(int i = 0; i < password.size() - 1; i++) {
        less = password[i] <= password[i+1];
        if (less == false) {
            break;
        }
    }
    return less;
};

bool TwoSequentuallyEqual::Validate(std::string password) {
    bool equal = false;
    for(int i = 0; i < password.size() - 1; i++) {
        equal = password[i] == password[i+1];
        if (equal) {
            break;
        }
    }
    return equal;
};

bool TwoEqual::Validate(std::string password) {
    bool twoEqual = false;

    for (auto&& gb : iter::groupby(password)) {
        int count = 0;
        for (auto&& s : gb.second) {
            count++;
        }

        if(count == 2) {
            twoEqual = true;
            break;
        }
    }

    return twoEqual;    
}

PasswordValidator::~PasswordValidator()
{
    this->validations.clear();
}

void PasswordValidator::setPasswordValidations(std::vector<Validation*> validations) {
    this->validations = validations;
}

bool PasswordValidator::validatePassword(int password){
    std::string passwordString = std::to_string(password);
    bool validPassword = false;
    for(int i = 0; i < validations.size(); i++) {
        validPassword = validations[i]->Validate(passwordString);
        if(!validPassword) {
            break;
        }
    }
    return validPassword;
};

int PasswordValidator::numberValidPassword(std::vector<int> passwords) {
    int numberValid = 0;
    std::for_each(std::begin(passwords), std::end(passwords),
     [this, &numberValid](int password){
         numberValid += this->validatePassword(password) ? 1 : 0;
     });
     return numberValid;
}
