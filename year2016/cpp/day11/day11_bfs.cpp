#include "day11.h"


vector<pair<ElevatorOption, FloorSimple> > generate_elevator(FloorSimple &old_hardware) {
    const auto pairs = generate_pairs(old_hardware);

    vector<pair<set<pair<string, HardwareType> >, FloorSimple> > levels;
    for (const auto &elevator_option: pairs) {
        FloorSimple hardware;
        std::ranges::set_difference(
            old_hardware,
            elevator_option,
            std::inserter(hardware, hardware.begin())
        );

        if (!is_valid(hardware)) {
            continue;
        }
        levels.emplace_back(elevator_option, hardware);
    }
    return levels;
}

vector<ElevatorOption> generate_pairs(FloorSimple &old_hardware) {
    vector<FloorSimple> pairs;
    vector temp(old_hardware.begin(), old_hardware.end());

    for (int i = 0; i < temp.size(); i++) {
        pairs.push_back({temp[i]});

        for (int j = i + 1; j < temp.size(); j++) {
            pairs.push_back({temp[i], temp[j]});
        }
    }

    return pairs;
}

bool is_floor_empty(FloorSimple &old_hardware) {
    return old_hardware.empty();
}

bool is_valid(FloorSimple &old_hardware) {
    set<string> generators;
    set<string> microchips;
    for (const auto &[name, type]: old_hardware) {
        if (type == GENERATOR) generators.insert(name);
        else microchips.insert(name);
    }

    return generators.empty() ||
           std::ranges::all_of(microchips,
                               [&](const string &microchip) {
                                   return generators.contains(microchip);
                               });
};

void add_hardware(const ElevatorOption &set, FloorSimple &hardware) {
    hardware.insert(set.begin(), set.end());
}

string generate_floor_cache(const FloorSimple &hardware) {
    std::string cache;

    int g = 0, m = 0;
    for (const auto &val: hardware | std::views::values) {
        switch (val) {
            case MICROCHIP:
                m += 1;
            case GENERATOR:
                g += 1;
        }
    }

    return std::to_string(g) + "g" + std::to_string(m) + "m";
}


StateSimple::StateSimple(
    const int steps,
    const int level,
    vector<FloorSimple> floors): _steps(steps), _level(level),
                                 _floors(std::move(floors)) {
}

StateSimple::StateSimple(
    const ElevatorOption &elevator,
    const int steps,
    const int level,
    vector<FloorSimple> floors) {
    _steps = steps;
    _level = level;
    _floors = std::move(floors);
    hydrate_from_elevator(elevator);
}


int StateSimple::steps() {
    return _steps;
}

void StateSimple::hydrate_from_elevator(const ElevatorOption &elevator) {
    FloorSimple &floor = _floors[_level];
    add_hardware(elevator, floor);
}

bool StateSimple::is_level_valid() {
    FloorSimple &floor = _floors[_level];
    return is_valid(floor);
}

bool StateSimple::all_on_level(const int level) {
    for (int i = 0; i < _floors.size(); ++i) {
        if (i == level) {
            continue;
        }

        if (FloorSimple &floor = _floors[i]; !is_floor_empty(floor)) {
            return false;
        }
    }


    return true;
}

vector<std::unique_ptr<IState> > StateSimple::next_states(const int max_level) {
    FloorSimple &floor = _floors.at(_level);
    auto elevators = generate_elevator(floor);
    vector<std::unique_ptr<IState> > states;
    const auto new_steps = _steps + 1;
    for (const auto &[elevator, floor]: elevators) {
        if (_level > 0) {
            auto floors = _floors;
            floors[_level] = floor;
            auto new_state = StateSimple(elevator, new_steps, _level - 1, floors);
            if (new_state.is_level_valid()) {
                states.push_back(std::make_unique<StateSimple>(std::move(new_state)));
            }
        }
        if (_level < max_level) {
            auto floors = _floors;
            floors[_level] = floor;
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

    for (int i = 0; i < _floors.size(); ++i) {
        FloorSimple &floor = _floors[i];
        const auto floor_cache = generate_floor_cache(floor);
        cache += std::to_string(i);
        cache += floor_cache;
        cache += '.';
    }

    return cache;
}

