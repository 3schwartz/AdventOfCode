#ifndef DAY_FOUR_FUNC_H
#define DAY_FOUR_FUNC_H

#include <vector>
#include <string>
#include<algorithm>

class PasswordFinder
{
    private:
        int from;
        int to;

    public:
        PasswordFinder(int from, int to);
        std::vector<int> getPasswords();
};

class Validation
{
    public:
        virtual bool Validate(std::string password) = 0;
};

class IncreasingValidation : public Validation
{
    public:
        bool Validate(std::string password);
};

class TwoSequentuallyEqual : public Validation
{
    public:
        bool Validate(std::string password);
};

class TwoEqual : public Validation
{
    public:
        bool Validate(std::string password);
};


class PasswordValidator
{
    private:
        std::vector<Validation*> validations;
    
    public:
        ~PasswordValidator();

        void setPasswordValidations(std::vector<Validation*> validations);

        bool validatePassword(int password);

        int numberValidPassword(std::vector<int> passwords);
};

#endif