#include <gtest/gtest.h>

#include <utility>
#include "day11.h"

#include <ranges>
using std::set;
using std::map;
using std::pair;
using std::string;
using std::vector;
using std::make_pair;
using std::tuple;
using std::move;

struct Elevator {
    set<string> _generators;
    set<string> _microchips;

    Elevator(set<string> generators, set<string> microchips) : _generators(
                                                                   std::move(generators)),
                                                               _microchips(std::move(microchips)) {
    }

    auto operator<=>(const Elevator &other) const = default;

    pair<set<string>, set<string> > generate_cache() {
        return make_pair(_generators, _microchips);
    }
};

class Floor {
    set<string> _generators;
    set<string> _microchips;

public:
    Floor(set<string> generators, set<string> microchips): _generators(std::move(generators)),
                                                           _microchips(std::move(microchips)) {
    }

    auto operator<=>(const Floor &other) const = default;

    void add_generators(const set<string> &generators) {
        _generators.insert(generators.begin(), generators.end());
    }

    void add_microchips(const set<string> &microchips) {
        _microchips.insert(microchips.begin(), microchips.end());
    }

    vector<pair<Elevator, Floor> > generate_elevator() {
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

    vector<Elevator> generate_pairs() const {
        vector<string> combined;
        auto generator_size = _generators.size();
        auto microchip_size = _microchips.size();
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

    pair<set<string>, set<string> > generate_cache() {
        return make_pair(_generators, _microchips);
    }

    [[nodiscard]] bool is_valid() const {
        return _generators.empty() || std::ranges::all_of(_microchips,
                                                         [&](const string &microchip) {
                                                             return _generators.contains(microchip);
                                                         });
    }

    bool is_empty() const {
        return _generators.empty() && _microchips.empty();
    }
};


using StateCache = tuple<int, pair<set<string>, set<string> >, map<int, pair<set<string>, set<string> > > >;

class State {
    int _steps;
    int _level;
    Elevator _elevator;
    map<int, Floor> _floors;

public:
    State(const int steps, const int level, Elevator elevator, map<int, Floor> floors): _steps(steps), _level(level),
        _elevator(std::move(elevator)),
        _floors(std::move(floors)) {
    }

    auto operator<=>(const State &other) const = default;

    int steps() const {
        return _steps;
    }

    void hydrate_from_elevator() {
        Floor &floor = _floors.at(_level);
        floor.add_generators(_elevator._generators);
        floor.add_microchips(_elevator._microchips);
        _elevator._generators.clear();
        _elevator._microchips.clear();
    }

    bool is_level_valid() {
        Floor &floor = _floors.at(_level);
        return floor.is_valid();
    }

    bool all_on_level(int level) {
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

    vector<State> next_states(const int max_level) {
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

    StateCache generate_cache() {
        map<int, pair<set<string>, set<string> > > floors;
        for (auto &[level, floor]: _floors) {
            auto floor_cache = floor.generate_cache();
            floors.emplace(level, floor_cache);
        }

        return std::make_tuple(_level, _elevator.generate_cache(), floors);
    }
};

class Facility {
public:
    static int order(State initial_state, int final_level) {
        set<StateCache> visited;
        vector<State> states = {std::move(initial_state)};

        while (!states.empty()) {
            auto state = states.back();
            states.pop_back();
            auto cache = state.generate_cache();
            if (!visited.insert(cache).second) {
                continue;
            }

            state.hydrate_from_elevator();
            if (state.is_level_valid()) {
                continue;
            }

            if (state.all_on_level(final_level)) {
                return state.steps();
            }

            for (auto new_state: state.next_states(final_level)) {
                states.push_back(std::move(new_state));
            }
        }
        return -1;
    }
};

// F4 .  .  .  .  .
// F3 .  .  .  LG .
// F2 .  HG .  .  .
// F1 E  .  HM .  LM

TEST(DAY11, FacilityOrder) {
    // Arrange
    const map<int, Floor> floors{
        {1, Floor({}, {"H", "L"})},
        {2, Floor({"H"}, {})},
        {3, Floor({"L"}, {})},
        {4, Floor({}, {})}
    };

    State initial_state = State(0, 1, Elevator({}, {}), floors);

    // Act
    int actual = Facility::order(initial_state, 3);


    // Assert
    int expected_steps = 11;
    EXPECT_EQ(actual, expected_steps);
}

TEST(DAY11, StateCacheEquals) {
    // Arrange
    auto first_state = State(1, 1, Elevator({"a"}, {"b"}),
                             {
                                 {1, Floor({"a"}, {"b"})},
                                 {2, Floor({"a"}, {"b"})}
                             });
    auto second_state = State(1, 1, Elevator({"a"}, {"b"}),
                              {
                                  {1, Floor({"a"}, {"b"})},
                                  {2, Floor({"a"}, {"b"})}
                              });

    // Act
    auto first_cache = first_state.generate_cache();
    auto second_cache = second_state.generate_cache();

    auto visited = set<StateCache>();
    auto [_, first_inserted] = visited.insert(first_cache);
    auto [_i, second_inserted] = visited.insert(second_cache);

    // Assert
    EXPECT_EQ(first_cache, second_cache);
    EXPECT_TRUE(first_inserted);
    EXPECT_FALSE(second_inserted);
}

TEST(DAY11, GenerateElevators) {
    // Arrange
    Floor floor({}, {});
    floor.add_generators({"a", "b"});
    floor.add_microchips({"a", "b"});

    // Act
    vector<pair<Elevator, Floor> > elevators = floor.generate_elevator();

    // Assert
    vector<Elevator> actual_elevators;
    actual_elevators.reserve(elevators.size());
    for (const auto &elevator: elevators | std::views::keys) {
        actual_elevators.push_back(elevator);
    }

    const vector<Elevator> expected{
        {{"a"}, {"a"}},
        {{"b"}, {"b"}},
        {{}, {"a", "b"}},
        {{}, {"a"}},
        {{}, {"b"}}
    };

    EXPECT_EQ(elevators.size(), 5);
    EXPECT_EQ(
        actual_elevators,
        expected
    );
    EXPECT_EQ(elevators[0].second, Floor({"b"}, {"b"}));
};

TEST(DAY11, GeneratePairs) {
    // Arrange
    Floor floor({}, {});
    floor.add_generators({"a", "b"});
    floor.add_microchips({"a", "b"});

    // Act
    const vector<Elevator> pairs = floor.generate_pairs();

    // Assert
    const vector<Elevator> expected{
        {{"a", "b"}, {}},
        {{"a"}, {"a"}},
        {{"a"}, {"b"}},
        {{"a"}, {}},
        {{"b"}, {"a"}},
        {{"b"}, {"b"}},
        {{"b"}, {}},
        {{}, {"a", "b"}},
        {{}, {"a"}},
        {{}, {"b"}}
    };
    EXPECT_EQ(pairs.size(), 4+3+2+1);
    EXPECT_EQ(
        pairs,
        expected
    );
};


class MapProperty {
    map<int, string> _my_map;

public:
    MapProperty(): _my_map{{1, "a"}} {
    }

    string get_first() {
        return _my_map.at(1);
    }

    map<int, string> get_map() {
        return _my_map;
    }
};

TEST(DAY11, MoveOrReference) {
    MapProperty map_property = MapProperty();
    auto map_prior = map_property.get_map();
    map_prior.at(1) = "b";
    EXPECT_EQ(map_property.get_first(), "a");
    EXPECT_EQ(map_prior.at(1), "b");
}

TEST(DAY11, SetEqual) {
    set<int> first = {1, 2, 3};
    set<int> second = {1, 2, 3};

    EXPECT_EQ(first, second);
}

TEST(DA11, map) {
    map<int, pair<set<string>, map<int, set<string> > > > first{
        {1, {{"set"}, {{1, {"a"}}, {2, {"b"}}}}}
    };
    map<int, pair<set<string>, map<int, set<string> > > > second{
        {1, {{"set"}, {{1, {"a"}}, {2, {"b"}}}}}
    };
    EXPECT_EQ(first, second);
    set<map<int, pair<set<string>, map<int, set<string> > > > > visited;
    visited.insert(first);
    auto [_, inserted] = visited.insert(second);
    EXPECT_EQ(visited.size(), 1);
    EXPECT_FALSE(inserted);
}

TEST(AdditionTest, HandlesPositiveNumbers) {
    EXPECT_EQ(add(2, 3), 5);
}

TEST(AdditionTest, HandlesNegativeNumbers) {
    EXPECT_EQ(add(-1, -2), -3);
}
