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

    auto start = std::chrono::high_resolution_clock::now();
    const auto initial_state = State(0, 1, Elevator({}, {}), floors);
    auto end = std::chrono::high_resolution_clock::now();

    std::cout << "Part 1: " << Facility::order(initial_state, 4);

    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Time taken: " << duration.count() << " milliseconds" << std::endl;
}
