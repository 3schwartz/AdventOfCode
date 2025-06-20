#include "day11.h"

FloorSimple::FloorSimple(set<pair<string, HardwareType> > hardware): _hardware(std::move(hardware)) {
};


vector<pair<ElevatorOption, FloorSimple> > FloorSimple::generate_elevator() {
    const auto pairs = generate_pairs();

    vector<pair<set<pair<string, HardwareType> >, FloorSimple> > levels;
    for (const auto &elevator_option: pairs) {
        set<pair<string, HardwareType> > hardware;
        std::ranges::set_difference(
            _hardware,
            elevator_option,
            std::inserter(hardware, hardware.begin())
        );
        auto new_floor = FloorSimple{hardware};
        if (!new_floor.is_valid()) {
            continue;
        }
        levels.emplace_back(elevator_option, new_floor);
    }
    return levels;
}

vector<ElevatorOption> FloorSimple::generate_pairs() const {
    vector<set<pair<string, HardwareType> > > pairs;
    vector<pair<string, HardwareType> > temp;
    for (const auto &p: _hardware) {
        pairs.push_back(set<pair<string, HardwareType> >{p});
        temp.push_back(p);
    }
    for (int i = 0; i < temp.size(); i++) {
        set<pair<string, HardwareType> > outer{temp[i]};
        for (int j = i + 1; j < temp.size(); j++) {
            auto inner = outer;
            inner.insert(temp[j]);
            pairs.push_back(inner);
        }
    }

    return pairs;
}

bool FloorSimple::is_empty() const {
    return _hardware.empty();
}

bool FloorSimple::is_valid() const {
    set<string> generators;
    set<string> microchips;
    for (const auto &[name, type]: _hardware) {
        if (type == GENERATOR) generators.insert(name);
        else microchips.insert(name);
    }

    return generators.empty() ||
           std::ranges::all_of(microchips,
                               [&](const string &microchip) {
                                   return generators.contains(microchip);
                               });
};

void FloorSimple::add_hardware(const ElevatorOption &set) {
    _hardware.insert(set.begin(), set.end());
}

string FloorSimple::generate_cache() const {
    std::string cache;

    for (const auto &pair: _hardware) {
        cache += pair.first;
        cache += std::to_string(pair.second);
        cache += 'x';
    }

    return cache;
}


StateSimple::StateSimple(
    const int steps,
    const int level,
    map<int, FloorSimple> floors): _steps(steps), _level(level),
                                   _floors(std::move(floors)) {
}

StateSimple::StateSimple(
    const ElevatorOption &elevator,
    const int steps,
    const int level,
    std::map<int, FloorSimple> floors) {
    _steps = steps;
    _level = level;
    _floors = std::move(floors);
    hydrate_from_elevator(elevator);
}

int StateSimple::steps() {
    return _steps;
}

void StateSimple::hydrate_from_elevator(const ElevatorOption &elevator) {
    FloorSimple &floor = _floors.at(_level);
    floor.add_hardware(elevator);
}

bool StateSimple::is_level_valid() const {
    const FloorSimple &floor = _floors.at(_level);
    return floor.is_valid();
}

bool StateSimple::all_on_level(const int level) {
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

vector<std::unique_ptr<IState> > StateSimple::next_states(const int max_level) {
    FloorSimple &floor = _floors.at(_level);
    auto elevators = floor.generate_elevator();
    vector<std::unique_ptr<IState> > states;
    const auto new_steps = _steps + 1;
    for (const auto &[elevator, floor]: elevators) {
        if (_level > 1) {
            map<int, FloorSimple> floors = _floors;
            floors.at(_level) = floor;
            auto new_state = StateSimple(elevator, new_steps, _level - 1, floors);
            if (new_state.is_level_valid()) {
                states.push_back(std::make_unique<StateSimple>(std::move(new_state)));
            }
        }
        if (_level < max_level) {
            map<int, FloorSimple> floors = _floors;
            floors.at(_level) = floor;
            auto new_state = StateSimple(elevator, new_steps, _level + 1, floors);
            if (new_state.is_level_valid()) {
                states.push_back(std::make_unique<StateSimple>(std::move(new_state)));
            }
        }
    }
    return states;
}

StateCache StateSimple::generate_cache() {
    string cache = std::to_string(_level) + ".";


    for (auto &[level, floor]: _floors) {
        const auto floor_cache = floor.generate_cache();
        cache += std::to_string(level);
        cache += floor_cache;
        cache += '.';
    }

    return cache;
}

