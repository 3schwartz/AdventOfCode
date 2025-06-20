#include <iostream>
#include "day11.h"
#include <map>
#include <utility>

using std::map;


void run_simple(map<int, FloorSimple> floors, const int part) {
    const auto initial_state = StateSimple(0, 1, std::move(floors));

    auto start = std::chrono::high_resolution_clock::now();
    auto steps = Facility::order(std::make_unique<StateSimple>(initial_state), 4);
    auto end = std::chrono::high_resolution_clock::now();

    std::cout << "Part " << part << ": " << steps << std::endl;

    auto duration = std::chrono::duration_cast<std::chrono::seconds>(end - start);
    std::cout << "Time taken: " << duration.count() << " seconds" << std::endl;
}

void run(map<int, Floor> floors, const int part) {
    const auto initial_state = State(0, 1, std::move(floors));

    auto start = std::chrono::high_resolution_clock::now();
    auto steps = Facility::order(std::make_unique<State>(initial_state), 4);
    auto end = std::chrono::high_resolution_clock::now();

    std::cout << "Part " << part << ": " << steps << std::endl;

    auto duration = std::chrono::duration_cast<std::chrono::seconds>(end - start);
    std::cout << "Time taken: " << duration.count() << " seconds" << std::endl;
}

// void run_dfs(map<int, Floor> floors, const int part) {
//     const auto initial_state = State(0, 1, std::move(floors));
//
//     auto start = std::chrono::high_resolution_clock::now();
//     auto steps = Facility::dfs_start(initial_state, 4);
//     auto end = std::chrono::high_resolution_clock::now();
//
//     std::cout << "DFS - Part " << part << ": " << steps << std::endl;
//
//     auto duration = std::chrono::duration_cast<std::chrono::seconds>(end - start);
//     std::cout << "Time taken: " << duration.count() << " seconds" << std::endl;
// }

int main() {
    const map<int, FloorSimple> floors_simple_1{
        {
            1,
            FloorSimple({
                {"thulium", GENERATOR}, {"plutonium", GENERATOR}, {"strontium", GENERATOR}, {"thulium", MICROCHIP}
            })
        },
        {2, FloorSimple({{"plutonium", MICROCHIP}, {"strontium", MICROCHIP}})},
        {
            3, FloorSimple({
                {"promethium", GENERATOR}, {"ruthenium", GENERATOR}, {"promethium", MICROCHIP}, {"ruthenium", MICROCHIP}
            })
        },
        {4, FloorSimple({})}
    };

    const map<int, Floor> floors_1{
        {1, Floor({"thulium", "plutonium", "strontium"}, {"thulium"})},
        {2, Floor({}, {"plutonium", "strontium"})},
        {3, Floor({"promethium", "ruthenium"}, {"promethium", "ruthenium"})},
        {4, Floor({}, {})}
    };

    run_simple(floors_simple_1, 1);

    run(floors_1, 1);

    // Upon entering the isolated containment area, however, you notice some extra parts on the first floor that weren't listed on the record outside:
    //
    // An elerium generator.
    // An elerium-compatible microchip.
    // A dilithium generator.
    // A dilithium-compatible microchip.
    const map<int, Floor> floors_2 = {
        {1, Floor({"thulium", "plutonium", "strontium", "elerium", "dilithium"}, {"thulium", "elerium", "dilithium"})},
        {2, Floor({}, {"plutonium", "strontium"})},
        {3, Floor({"promethium", "ruthenium"}, {"promethium", "ruthenium"})},
        {4, Floor({}, {})}
    };

    run(floors_2, 2);

    // run_dfs(floors_1, 1);
}


