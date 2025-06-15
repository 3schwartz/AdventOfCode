#include <iostream>
#include "day11.h"
#include <map>
using std::map;

int main() {
    const map<int, Floor> floors{
        {1, Floor({"thulium", "plutonium", "strontium"}, {"thulium"})},
        {2, Floor({}, {"plutonium", "strontium"})},
        {3, Floor({"promethium", "ruthenium"}, {"promethium", "ruthenium"})},
        {4, Floor({}, {})}
    };

    const auto initial_state = State(0, 1, Elevator({}, {}), floors);

    // Act
    std::cout << "Part 1: " << Facility::order(initial_state, 4);
}
