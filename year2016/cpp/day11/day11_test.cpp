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
    int _level;
    set<string> _generators;
    set<string> _microchips;

public:
    explicit Floor(const int level) : _level(level) {
    }

    Floor(const int level, set<string> generators, set<string> microchips): _level(level),
                                                                            _generators(std::move(generators)),
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
            bool valid = true;
            for (const string &microchip: new_microchips) {
                if (!new_generators.contains(microchip)) {
                    valid = false;
                    break;
                }
            }
            if (!valid) {
                continue;
            }
            levels.emplace_back(Elevator{generators, microchips}, Floor{_level, new_generators, new_microchips});
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

    pair<int, pair<set<string>, set<string> > > generate_cache() {
        return make_pair(_level, pair{_generators, _microchips});
    }
};


using StateCache = tuple<int, pair<set<string>, set<string> >, map<int, pair<set<string>, set<string> > > >;

class State {
    int _level;
    Elevator _elevator;
    map<int, Floor> _floors;

public:
    State(const int level, Elevator elevator, map<int, Floor> floors): _level(level),
                                                                       _elevator(std::move(elevator)),
                                                                       _floors(std::move(floors)) {
    }

    auto operator<=>(const State &other) const = default;

    StateCache generate_cache() {
        map<int, pair<set<string>, set<string> > > floors;
        for (auto &floor: _floors | std::views::values) {
            auto [fst, snd] = floor.generate_cache();
            floors.emplace(fst, snd);
        }

        return std::make_tuple(_level, _elevator.generate_cache(), floors);
    }
};

TEST(DAY11, StateCacheEquals) {
    // Arrange
    auto first_state = State(1, Elevator({"a"}, {"b"}),
                             {
                                 {1, Floor(1, {"a"}, {"b"})},
                                 {2, Floor(2, {"a"}, {"b"})}
                             });
    auto second_state = State(1, Elevator({"a"}, {"b"}),
                              {
                                  {1, Floor(1, {"a"}, {"b"})},
                                  {2, Floor(2, {"a"}, {"b"})}
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
    Floor floor(1);
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
    EXPECT_EQ(elevators[0].second, Floor(1, {"b"}, {"b"}));
};

TEST(DAY11, GeneratePairs) {
    // Arrange
    Floor floor(1);
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
