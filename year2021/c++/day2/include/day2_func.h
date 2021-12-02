#ifndef DAY_ONE_FUNC_H
#define DAY_ONE_FUNC_H

#include <vector>
#include <string>

class Move {
    private:
        std::vector<std::string> GetSplit(std::string move);

    public:
        std::string direction;
        int step;

        Move(std::string move);
};

#endif