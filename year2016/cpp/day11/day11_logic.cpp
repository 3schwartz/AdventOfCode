#include "day11.h"


Elevator::Elevator(set<string> generators, set<string> microchips)
    : _generators(std::move(generators)), _microchips(std::move(microchips)) {
}

pair<set<string>, set<string> > Elevator::generate_cache() {
    return make_pair(_generators, _microchips);
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

pair<set<string>, set<string> > Floor::generate_cache() {
    return make_pair(_generators, _microchips);
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

State::State(const int steps, const int level, Elevator elevator, map<int, Floor> floors): _steps(steps), _level(level),
    _elevator(std::move(elevator)),
    _floors(std::move(floors)) {
}

int State::steps() const {
    return _steps;
}

void State::hydrate_from_elevator() {
    Floor &floor = _floors.at(_level);
    floor.add_generators(_elevator._generators);
    floor.add_microchips(_elevator._microchips);
    _elevator._generators.clear();
    _elevator._microchips.clear();
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

vector<State> State::next_states(const int max_level) {
    Floor &floor = _floors.at(_level);
    auto elevators = floor.generate_elevator();
    vector<State> states;
    auto new_steps = _steps + 1;
    for (const auto &[elevator, floor]: elevators) {
        if (_level > 1) {
            map<int, Floor> floors = _floors;
            floors.at(_level) = floor;
            states.emplace_back(new_steps, _level - 1, elevator, floors);
        }
        if (_level < max_level) {
            map<int, Floor> floors = _floors;
            floors.at(_level) = floor;
            states.emplace_back(new_steps, _level + 1, elevator, floors);
        }
    }
    return states;
}

StateCache State::generate_cache() {
    map<int, pair<set<string>, set<string> > > floors;
    for (auto &[level, floor]: _floors) {
        auto floor_cache = floor.generate_cache();
        floors.emplace(level, floor_cache);
    }

    return std::make_pair(_level, floors);
}
