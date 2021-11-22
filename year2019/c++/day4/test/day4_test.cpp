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

TEST_CASE("Correct range") {
    // Arrange
    int from = 1;
    int to = 3;
    PasswordFinder passwordFinder(from, to);
    std::vector<int> passwords = passwordFinder.getPasswords();

    CHECK(passwords.size() == 3);
    CHECK(passwords[0] == 1);
    CHECK(passwords[1] == 2);
    CHECK(passwords[2] == 3);
}