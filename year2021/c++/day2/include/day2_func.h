#ifndef DAY_ONE_FUNC_H
#define DAY_ONE_FUNC_H

#include <vector>
#include <string>

using namespace std;

class Move {
    private:
        std::vector<std::string> GetSplit(std::string move);

    public:
        std::string direction;
        int step;

        Move(std::string move);
};

class Validator
{
    public:
        virtual int GetPosition(vector<Move> moves) = 0;
};

class RouteCalculator : public Validator
{
    public:
        int GetPosition(vector<Move> moves);
};

class AimCalculator : public Validator
{
    public:
        int GetPosition(vector<Move> moves);
};

#endif