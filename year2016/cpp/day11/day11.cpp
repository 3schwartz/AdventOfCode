#include <iostream>
#include "day11.h"
#include <map>
#include <utility>
#include <vector>
#include <set>
#include <string>
#include <sstream>
#include <fstream>

using std::map;
using std::set;
using std::vector;

void run_simple(vector<FloorSimple> floors, const int part) {
    const auto initial_state = StateSimple(0, 0, std::move(floors));

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = Facility::order(std::make_unique<StateSimple>(initial_state), 3);
    const auto end = std::chrono::high_resolution_clock::now();

    std::cout << "Part " << part << ": " << steps << std::endl;

    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Time taken: " << duration.count() << " milliseconds" << std::endl;
}

void run(map<int, Floor> floors, const int part) {
    const auto initial_state = State(0, 1, std::move(floors));

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = Facility::order(std::make_unique<State>(initial_state), 4);
    const auto end = std::chrono::high_resolution_clock::now();

    std::cout << "Part " << part << ": " << steps << std::endl;

    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Time taken: " << duration.count() << " milliseconds" << std::endl;
}

void run_dfs(map<int, Floor> floors, const int part) {
    const auto initial_state = State(0, 1, std::move(floors));

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = Facility::dfs_start(std::make_unique<State>(initial_state), 4);
    const auto end = std::chrono::high_resolution_clock::now();

    std::cout << "DFS - Part " << part << ": " << steps << std::endl;

    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Time taken: " << duration.count() << " milliseconds" << std::endl;
}


using Item = std::pair<std::string, char>;
using FloorV2 = std::set<Item>;
using StateV2 = std::tuple<int, int, std::vector<FloorV2> >;

vector<set<std::pair<std::string, char> > > parse_states(const vector<std::string> &data) {
    vector<set<std::pair<std::string, char> > > floors;

    for (const auto &line: data) {
        set<std::pair<std::string, char> > floor;

        size_t pos = 0;
        size_t start = 0;
        vector<std::string> parts;

        while ((pos = line.find(" a ", start)) != std::string::npos) {
            parts.push_back(line.substr(start, pos - start));
            start = pos + 3;
        }
        parts.push_back(line.substr(start)); // Add the last part

        for (size_t i = 1; i < parts.size(); ++i) {
            std::istringstream iss(parts[i]);
            std::string name, type;
            iss >> name >> type;

            std::string n = name.substr(0, 3);
            char t = type[0];
            floor.insert({n, t});
        }

        floors.push_back(floor);
    }

    return floors;
}

std::vector<std::string> load_from_file(const std::string &filename = "day11_data.txt") {
    std::vector<std::string> lines;
    std::ifstream file(filename);

    if (!file.is_open()) {
        std::cerr << "Error: Unable to open file " << filename << std::endl;
        return lines;
    }

    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    return lines;
}


bool is_done(const std::vector<FloorV2> &floors) {
    return std::all_of(floors.begin(), floors.begin() + 3,
                       [](const FloorV2 &f) { return f.empty(); });
}

bool is_floor_safe(const FloorV2 &floor) {
    std::set<std::string> generators, microchips;
    for (const auto &[fst, snd]: floor) {
        if (snd == 'g') generators.insert(fst);
        else if (snd == 'm') microchips.insert(fst);
    }
    if (generators.empty()) return true;
    for (const auto &chip: microchips) {
        if (!generators.contains(chip)) return false;
    }
    return true;
}

std::vector<std::vector<Item> > get_combinations(const FloorV2 &floor) {
    std::vector<Item> items(floor.begin(), floor.end());
    std::vector<std::vector<Item> > combos;

    for (size_t i = 0; i < items.size(); ++i) {
        combos.push_back({items[i]});
        for (size_t j = i + 1; j < items.size(); ++j) {
            combos.push_back({items[i], items[j]});
        }
    }
    return combos;
}

std::string get_floors_status(int elevator, const std::vector<FloorV2> &floors) {
    std::string key = std::to_string(elevator) + ":";
    for (const auto &floor: floors) {
        int g = 0, m = 0;
        for (const auto &item: floor) {
            if (item.second == 'g') ++g;
            else if (item.second == 'm') ++m;
        }
        key += std::to_string(g) + "g" + std::to_string(m) + "m|";
    }
    return key;
}

std::vector<StateV2> get_possible_steps(const StateV2 &state) {
    auto [count, elevator, floors] = state;

    std::vector<StateV2> next_states;
    const auto combos = get_combinations(floors[elevator]);

    for (const int dir: {-1, 1}) {
        int next_elevator = elevator + dir;
        if (next_elevator < 0 || next_elevator >= 4) continue;

        for (const auto &move: combos) {
            std::vector<FloorV2> new_floors = floors;
            for (const auto &item: move) {
                new_floors[elevator].erase(item);
                new_floors[next_elevator].insert(item);
            }

            if (is_floor_safe(new_floors[elevator]) && is_floor_safe(new_floors[next_elevator])) {
                next_states.emplace_back(count + 1, next_elevator, new_floors);
            }
        }
    }

    return next_states;
}

int run_steps(const std::vector<FloorV2> &floors) {
    std::set<std::string> seen;
    std::queue<StateV2> q;
    q.emplace(0, 0, floors);

    while (!q.empty()) {
        auto [count, elevator, current_floors] = q.front();
        q.pop();

        if (is_done(current_floors)) return count;

        std::string status = get_floors_status(elevator, current_floors);
        if (seen.contains(status)) continue;
        seen.insert(status);

        for (const auto &next_state: get_possible_steps({count, elevator, current_floors})) {
            q.push(next_state);
        }
    }

    return -1;
}

int part1(const std::vector<std::string> &data) {
    const auto floors = parse_states(data);
    return run_steps(floors);
}

int part2(const std::vector<std::string> &data) {
    auto floors = parse_states(data);

    floors[0].insert({"ele", 'g'});
    floors[0].insert({"ele", 'm'});
    floors[0].insert({"dil", 'g'});
    floors[0].insert({"dil", 'm'});

    const auto start = std::chrono::high_resolution_clock::now();
    const auto steps = run_steps(floors);
    const auto end = std::chrono::high_resolution_clock::now();
    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Time taken default containers: " << duration.count() << " milliseconds" << std::endl;
    return steps;
}

int main() {
    const auto data = load_from_file("../../data/day11_data.txt");

    const int steps1 = part1(data);
    std::cout << "Part 1: " << steps1 << " steps\n";

    const int steps2 = part2(data);
    std::cout << "Part 2: " << steps2 << " steps\n";


    const vector<FloorSimple> floors_simple_1{
        FloorSimple({
            {"thulium", GENERATOR}, {"plutonium", GENERATOR}, {"strontium", GENERATOR}, {"thulium", MICROCHIP}
        }),
        FloorSimple({{"plutonium", MICROCHIP}, {"strontium", MICROCHIP}}),
        FloorSimple({
            {"promethium", GENERATOR}, {"ruthenium", GENERATOR}, {"promethium", MICROCHIP}, {"ruthenium", MICROCHIP}
        }),
        FloorSimple({})
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

    const vector floors_simple_2{
        FloorSimple({
            {"thulium", GENERATOR}, {"plutonium", GENERATOR}, {"strontium", GENERATOR}, {"thulium", MICROCHIP},
            {"elerium", GENERATOR},
            {"elerium", MICROCHIP},
            {"dilithium", GENERATOR},
            {"dilithium", MICROCHIP}
        }),
        FloorSimple({{"plutonium", MICROCHIP}, {"strontium", MICROCHIP}}),
        FloorSimple({
            {"promethium", GENERATOR}, {"ruthenium", GENERATOR}, {"promethium", MICROCHIP}, {"ruthenium", MICROCHIP}
        }),
        FloorSimple({})
    };

    run_simple(floors_simple_2, 2);

    run_dfs(floors_1, 1);
    return
            0;
}


