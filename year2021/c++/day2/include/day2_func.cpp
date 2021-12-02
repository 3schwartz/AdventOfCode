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