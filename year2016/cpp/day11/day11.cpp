#include <iostream>
#include "day11.h"
#include <map>
#include <utility>
#include <vector>
#include <set>
#include <string>
#include <sstream>
#include <fstream>
#include "day11_v2.h"


using std::cout;
using std::endl;
using std::vector;

void run_simple(vector<FloorSimple> floors, const int part) {
    const auto initial_state = StateSimple(0, 0, std::move(floors));

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = Facility::order(std::make_unique<StateSimple>(initial_state), 3);
    const auto end = std::chrono::high_resolution_clock::now();

    cout << "Part " << part << ": " << steps << endl;

    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    cout << "Time taken: " << duration.count() << " milliseconds" << endl;
}

void run(map<int, Floor> floors, const int part) {
    const auto initial_state = State(0, 1, std::move(floors));

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = Facility::order(std::make_unique<State>(initial_state), 4);
    const auto end = std::chrono::high_resolution_clock::now();

    cout << "Part " << part << ": " << steps << endl;

    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    cout << "Time taken: " << duration.count() << " milliseconds" << endl;
}

void run_dfs(map<int, Floor> floors, const int part) {
    const auto initial_state = State(0, 1, std::move(floors));

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = Facility::dfs_start(std::make_unique<State>(initial_state), 4);
    const auto end = std::chrono::high_resolution_clock::now();

    cout << "DFS - Part " << part << ": " << steps << endl;

    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    cout << "Time taken: " << duration.count() << " milliseconds" << endl;
}


void run_v2() {
    cout << endl << "############## V2 ####################" << endl;


    const auto data = load_from_file("../../data/day11_data.txt");

    const int steps1 = part1(data);
    cout << "Part 1: " << steps1 << " steps\n";

    const int steps2 = part2(data);
    cout << "Part 2: " << steps2 << " steps\n";

    cout << "############## V2 ####################" << endl << endl;
}

int main() {
    run_v2();

    auto floors_simple = floors_simple_load_from_file("../../data/day11_data.txt");

    auto floors = Floor::load_from_file("../../data/day11_data.txt");

    run_simple(floors_simple, 1);
    run(floors, 1);
    run_dfs(floors, 1);

    // Upon entering the isolated containment area, however, you notice some extra parts on the first floor that weren't listed on the record outside:
    //
    // An elerium generator.
    // An elerium-compatible microchip.
    // A dilithium generator.
    // A dilithium-compatible microchip.
    floors.at(1).add_generators({"elerium", "dilithium"});
    floors.at(1).add_microchips({"elerium", "dilithium"});

    run(floors, 2);

    floors_simple[0].insert({
        {"elerium", GENERATOR},
        {"elerium", MICROCHIP},
        {"dilithium", GENERATOR},
        {"dilithium", MICROCHIP}
    });
    run_simple(floors_simple, 2);

    return 0;
}


