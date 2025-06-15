#include <gtest/gtest.h>
#include <ranges>
#include "day11.h"

using std::map;
using std::set;
using std::vector;
using std::pair;
using std::string;
using std::make_pair;
using std::tuple;
using std::move;

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
    int actual = Facility::order(initial_state, 4);


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
    Floor floor({"a", "b"}, {"a", "b"});

    // Act
    vector<pair<Elevator, Floor> > elevators = floor.generate_elevator();

    // Assert
    vector<Elevator> actual_elevators;
    actual_elevators.reserve(elevators.size());
    for (const auto &elevator: elevators | std::views::keys) {
        actual_elevators.push_back(elevator);
    }

    const vector<Elevator> expected{
        {{"a", "b"}, {}},
        {{"a"}, {"a"}},
        {{"b"}, {"b"}},
        {{}, {"a", "b"}},
        {{}, {"a"}},
        {{}, {"b"}}
    };

    EXPECT_EQ(elevators.size(), 6);
    EXPECT_EQ(
        actual_elevators,
        expected
    );
    EXPECT_EQ(elevators[1].second, Floor({"b"}, {"b"}));
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
    auto map_property = MapProperty();
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

TEST(DAY11, map) {
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
