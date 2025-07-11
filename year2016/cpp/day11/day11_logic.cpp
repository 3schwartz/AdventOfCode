#include <fstream>
#include <sstream>

#include "day11.h"
#include "../common/common.h"

using std::ifstream;

Elevator::Elevator(set<string> generators, set<string> microchips)
    : _generators(std::move(generators)), _microchips(std::move(microchips)) {
}


map<int, Floor> Floor::load_from_file(const std::string &filename) {
    const auto lines = read_lines(filename);

    map<int, Floor> floors;

    for (size_t i = 0; i < lines.size(); i++) {
        auto floor = Floor(lines[i]);
        floors.insert({i + 1, floor});
    }
    return floors;
}

Floor::Floor(const string &input) {
    size_t pos = 0;
    size_t start = 0;
    vector<string> parts;

    while ((pos = input.find(" a ", start)) != string::npos) {
        parts.push_back(input.substr(start, pos - start));
        start = pos + 3;
    }
    parts.push_back(input.substr(start));

    for (size_t i = 1; i < parts.size(); ++i) {
        std::istringstream iss(parts[i]);
        string hardware, type;
        iss >> hardware >> type;

        switch (type[0]) {
            case 'm':
                pos = hardware.find("-compatible");
                _microchips.insert(hardware.substr(0, pos));
                break;
            case 'g':
                _generators.insert(hardware);
                break;
            default:
                throw std::invalid_argument("Invalid type: " + type);
        }
    }
}

Floor::Floor(set<string> generators, set<string> microchips): _generators(std::move(generators)),
                                                              _microchips(std::move(microchips)) {
}

void Floor::add_generators(const set<string> &generators) {
    _generators.insert(generators.begin(), generators.end());
}

void Floor::add_microchips(const set<string> &microchips) {
    _microchips.insert(microchips.begin(), microchips.end());
}

static HardwareType hardware_type_from_char(char c) {
    switch (c) {
        case 'm':
            return MICROCHIP;
        case 'g':
            return GENERATOR;
        default:
            throw std::invalid_argument("Invalid type: " + std::to_string(c));
    }
}

vector<FloorSimple> floors_simple_load_from_file(const std::string &filename) {
    const auto lines = read_lines(filename);

    vector<FloorSimple> floors;
    for (const auto &line: lines) {
        auto floor = floor_simple_from_string(line);
        floors.push_back(floor);
    }
    return floors;
}

FloorSimple floor_simple_from_string(const string &input) {
    size_t pos = 0;
    size_t start = 0;
    vector<string> parts;

    while ((pos = input.find(" a ", start)) != string::npos) {
        parts.push_back(input.substr(start, pos - start));
        start = pos + 3;
    }
    parts.push_back(input.substr(start));

    FloorSimple floor;
    for (size_t i = 1; i < parts.size(); ++i) {
        std::istringstream iss(parts[i]);
        string hardware, type;
        iss >> hardware >> type;

        auto hardware_type = hardware_type_from_char(type[0]);
        pos = hardware.find("-compatible");
        auto name_truncated = (pos != string::npos) ? hardware.substr(0, pos) : hardware;
        floor.insert({name_truncated, hardware_type});
    }

    return floor;
}

vector<pair<Elevator, Floor> > Floor::generate_elevator() {
    const auto pairs = generate_pairs();
    vector<pair<Elevator, Floor> > levels;
    for (const auto &[elevator_generator, elevator_microchip]: pairs) {
        auto generators = elevator_generator;
        auto microchips = elevator_microchip;

        set<string> new_generators;
        set<string> new_microchips;

        std::ranges::set_difference(_generators, generators,
                                    std::inserter(new_generators, new_generators.begin()));
        std::ranges::set_difference(_microchips, microchips,
                                    std::inserter(new_microchips, new_microchips.begin()));
        auto new_floor = Floor{new_generators, new_microchips};
        if (!new_floor.is_valid()) {
            continue;
        }

        levels.emplace_back(Elevator{generators, microchips}, new_floor);
    }
    return levels;
}

vector<Elevator> Floor::generate_pairs() const {
    vector<string> combined;
    const auto generator_size = _generators.size();
    const auto microchip_size = _microchips.size();
    combined.reserve(generator_size + microchip_size);
    for (const string &generator: _generators) combined.emplace_back(generator);
    for (const string &microchip: _microchips) combined.emplace_back(microchip);

    vector<Elevator> pairs;
    for (int i = 0; i < combined.size(); i++) {
        const string &first = combined[i];
        for (int j = i + 1; j < combined.size(); j++) {
            set<string> _g;
            set<string> _m;
            const string &second = combined[j];
            (i < generator_size ? _g : _m).insert(first);
            (j < generator_size ? _g : _m).insert(second);
            pairs.emplace_back(_g, _m);
        }
        set<string> _g, _m;
        (i < generator_size ? _g : _m).insert(first);
        pairs.emplace_back(_g, _m);
    }
    return pairs;
}

string Floor::generate_cache() const {
    std::string cache;

    return std::to_string(_generators.size()) + "g" + std::to_string(_microchips.size()) + "m";
}

[[nodiscard]] bool Floor::is_valid() const {
    return _generators.empty() ||
           std::ranges::all_of(_microchips,
                               [&](const string &microchip) {
                                   return _generators.contains(microchip);
                               });
}

[[nodiscard]] bool Floor::is_empty() const {
    return _generators.empty() && _microchips.empty();
}

State::State(const int steps, const int level, map<int, Floor> floors): _steps(steps), _level(level),
                                                                        _floors(std::move(floors)) {
}

State::State(const Elevator &elevator, const int steps, const int level, std::map<int, Floor> floors) {
    _steps = steps;
    _level = level;
    _floors = std::move(floors);
    hydrate_from_elevator(elevator);
}

int State::steps() {
    return _steps;
}

void State::hydrate_from_elevator(const Elevator &elevator) {
    Floor &floor = _floors.at(_level);
    floor.add_generators(elevator._generators);
    floor.add_microchips(elevator._microchips);
}

bool State::is_level_valid() const {
    const Floor &floor = _floors.at(_level);
    return floor.is_valid();
}

bool State::all_on_level(int level) {
    for (const auto &[floor_level, floor]: _floors) {
        if (floor_level == level) {
            continue;
        }
        if (!floor.is_empty()) {
            return false;
        }
    }

    return true;
}

vector<std::unique_ptr<IState> > State::next_states(const int max_level) {
    Floor &floor = _floors.at(_level);
    auto elevators = floor.generate_elevator();
    vector<std::unique_ptr<IState> > states;
    const auto new_steps = _steps + 1;
    for (const auto &[elevator, floor]: elevators) {
        if (_level > 1) {
            map<int, Floor> floors = _floors;
            floors.at(_level) = floor;
            auto new_state = State(elevator, new_steps, _level - 1, floors);
            if (new_state.is_level_valid()) {
                states.push_back(std::make_unique<State>(std::move(new_state)));
            }
        }
        if (_level < max_level) {
            map<int, Floor> floors = _floors;
            floors.at(_level) = floor;
            auto new_state = State(elevator, new_steps, _level + 1, floors);
            if (new_state.is_level_valid()) {
                states.push_back(std::make_unique<State>(std::move(new_state)));
            }
        }
    }
    return states;
}

StateCache State::generate_cache() {
    string cache = std::to_string(_level) + ".";


    for (auto &[level, floor]: _floors) {
        auto floor_cache = floor.generate_cache();
        cache += floor_cache;
    }

    return cache;
}

int Facility::dfs_start(std::unique_ptr<IState> state, const int final_level) {
    map<StateCache, int> optimal;
    map<StateCache, int> best_seen;
    int global_min_steps = INT_MAX;
    return dfs_iterate(state, optimal, best_seen, final_level, global_min_steps);
}

int Facility::dfs_iterate(
    const std::unique_ptr<IState> &state,
    map<StateCache, int> &optimal,
    map<StateCache, int> &best_seen,
    int final_level,
    int &global_min_steps) {
    const auto cache = state->generate_cache();
    const int current_steps = state->steps();


    if (current_steps >= global_min_steps) {
        return INT_MAX;
    }

    if (best_seen.contains(cache) && best_seen[cache] <= current_steps) {
        return INT_MAX;
    }

    best_seen[cache] = current_steps;

    if (state->all_on_level(final_level)) {
        global_min_steps = current_steps;
        optimal[cache] = 0;
        return current_steps;
    }

    if (optimal.contains(cache)) {
        return current_steps + optimal[cache];
    }

    int min_total_steps = INT_MAX;

    for (auto &&next_state: state->next_states(final_level)) {
        int result = dfs_iterate(next_state, optimal, best_seen, final_level, global_min_steps);
        if (result < min_total_steps) {
            min_total_steps = result;
        }
    }

    if (min_total_steps != INT_MAX) {
        optimal[cache] = min_total_steps - current_steps;
    }

    return min_total_steps;
}

int Facility::order(std::unique_ptr<IState> initial_state, int final_level) {
    set<StateCache> visited;
    std::queue<std::unique_ptr<IState> > states;
    states.push(std::move(initial_state));

    while (!states.empty()) {
        const auto state = std::move(states.front());
        states.pop();

        auto cache = state->generate_cache();
        if (!visited.insert(cache).second) {
            continue;
        }

        if (state->all_on_level(final_level)) {
            return state->steps();
        }

        for (auto &&new_state: state->next_states(final_level)) {
            states.push(std::move(new_state));
        }
    }
    return -1;
}
