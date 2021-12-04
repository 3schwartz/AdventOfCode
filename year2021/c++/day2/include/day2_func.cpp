#include "day2_func.h"

#include <sstream>

Move::Move(std::string move) {
    std::vector<std::string> strings = GetSplit(move);
    direction = strings[0];
    step = stoi(strings[1]);
}

std::vector<std::string> Move::GetSplit(std::string move){
    std::istringstream iss(move);
    std::string item;
    std::vector<std::string> strings;
    while (std::getline(iss, item, ' ')) {
        strings.push_back(item);
    };
    return strings;
}

int RouteCalculator::GetPosition(vector<Move> moves){
    int position = 0;
    int debt = 0;

    for(Move &move : moves) {
        if(move.direction == "forward"){
            position += move.step;
        }
        if(move.direction == "down"){
            debt += move.step;
        }
        if(move.direction == "up"){
            debt -= move.step;
        }                                
    }
    return debt * position;
};

int AimCalculator::GetPosition(vector<Move> moves){
    int position = 0;
    int debt = 0;
    int aim = 0;

    for(Move &move : moves) {
        if(move.direction == "forward"){
            position += move.step;
            debt += aim * move.step;
        }
        if(move.direction == "down"){
            aim += move.step;
        }
        if(move.direction == "up"){
            aim -= move.step;
        }                                
    }
    return debt * position;
};