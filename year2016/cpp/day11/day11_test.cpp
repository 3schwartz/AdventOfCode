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

TEST(DAY11, FacilityOrderSimple) {
    // Arrange
    const map<int, FloorSimple> floors{
        {1, FloorSimple({{"H", MICROCHIP}, {"L", MICROCHIP}})},
        {2, FloorSimple({{"H", GENERATOR}})},
        {3, FloorSimple({{"L", GENERATOR}})},
        {4, FloorSimple({})}
    };


    const auto initial_state = StateSimple(0, 1, floors);

    // Act
    const int actual = Facility::order(std::make_unique<StateSimple>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 11;
    EXPECT_EQ(actual, expected_steps);
}

TEST(DAY11, GenerateElevatorsSimple) {
    // Arrange
    FloorSimple floor({
            {"a", GENERATOR},
            {"a", MICROCHIP},
            {"b", MICROCHIP},
            {"b", GENERATOR}
        }
    );

    // Act
    auto elevators = floor.generate_elevator();

    // Assert
    vector<ElevatorOption> actual_elevators;
    actual_elevators.reserve(elevators.size());
    for (const auto &elevator: elevators | std::views::keys) {
        actual_elevators.push_back(elevator);
    }

    const vector<ElevatorOption> expected{
        {{"a", MICROCHIP}},
        {{"b", MICROCHIP}},
        {{"a", GENERATOR}, {"a", MICROCHIP}},
        {{"a", GENERATOR}, {"b", GENERATOR}},
        {{"a", MICROCHIP}, {"b", MICROCHIP}},
        {{"b", GENERATOR}, {"b", MICROCHIP}}

    };

    EXPECT_EQ(elevators.size(), 6);
    EXPECT_EQ(
        actual_elevators,
        expected
    );
    EXPECT_EQ(elevators[1].second, FloorSimple({
                  {"a", GENERATOR},
                  {"a", MICROCHIP},
                  {"b", GENERATOR}
                  }));
};

TEST(DAY11, GeneratePairsSimple) {
    // Arrange
    FloorSimple floor({
        {"a", GENERATOR},
        {"b", GENERATOR},
        {"a", MICROCHIP},
        {"b", MICROCHIP}
    });

    // Act
    const auto pairs = floor.generate_pairs();

    // Assert
    const vector<set<pair<string, HardwareType> > > expected{
        {{"a", GENERATOR}},
        {{"a", MICROCHIP}},
        {{"b", GENERATOR}},
        {{"b", MICROCHIP}},
        {{"a", GENERATOR}, {"a", MICROCHIP}},
        {{"a", GENERATOR}, {"b", GENERATOR}},
        {{"a", GENERATOR}, {"b", MICROCHIP}},
        {{"a", MICROCHIP}, {"b", GENERATOR}},
        {{"a", MICROCHIP}, {"b", MICROCHIP}},
        {{"b", GENERATOR}, {"b", MICROCHIP}},
    };
    EXPECT_EQ(pairs.size(), 4+3+2+1);
    EXPECT_EQ(
        pairs,
        expected
    );
};


// F4 .  .  .  .  .
// F3 E  HG HM LG LM
// F2 .  .  .  .  .
// F1 .  .  .  .  .
TEST(DAY11, FacilityOrderDFS5) {
    // Arrange
    const map<int, Floor> floors{
        {1, Floor({}, {})},
        {2, Floor({}, {})},
        {3, Floor({"H", "L"}, {"H", "L"})},
        {4, Floor({}, {})}
    };

    const auto initial_state = State(0, 3, floors);

    // Act
    const int actual = Facility::dfs_start(std::make_unique<State>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 5;
    EXPECT_EQ(actual, expected_steps);
}

// F4 E  .  HM .  LM
// F3 .  HG .  LG .
// F2 .  .  .  .  .
// F1 .  .  .  .  .
TEST(DAY11, FacilityOrderDFS4) {
    // Arrange
    const map<int, Floor> floors{
        {1, Floor({}, {})},
        {2, Floor({}, {})},
        {3, Floor({"H", "L"}, {})},
        {4, Floor({}, {"H", "L"})}
    };

    const auto initial_state = State(0, 4, floors);

    // Act
    const int actual = Facility::dfs_start(std::make_unique<State>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 4;
    EXPECT_EQ(actual, expected_steps);
}

// F4 .  .  .  .  LM
// F3 E  HG HM LG .
// F2 .  .  .  .  .
// F1 .  .  .  .  .
TEST(DAY11, FacilityOrderDFS3) {
    // Arrange
    const map<int, Floor> floors{
        {1, Floor({}, {})},
        {2, Floor({}, {})},
        {3, Floor({"H", "L"}, {"H"})},
        {4, Floor({}, {"L"})}
    };

    const auto initial_state = State(0, 3, floors);

    // Act
    const int actual = Facility::dfs_start(std::make_unique<State>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 3;
    EXPECT_EQ(actual, expected_steps);
}

// F4 E  HG .  LG LM
// F3 .  .  HM .  .
// F2 .  .  .  .  .
// F1 .  .  .  .  .
TEST(DAY11, FacilityOrderDFS2) {
    // Arrange
    const map<int, Floor> floors{
        {1, Floor({}, {})},
        {2, Floor({}, {})},
        {3, Floor({}, {"H"})},
        {4, Floor({"H", "L"}, {"L"})}
    };

    const auto initial_state = State(0, 4, floors);

    // Act
    const int actual = Facility::dfs_start(std::make_unique<State>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 2;
    EXPECT_EQ(actual, expected_steps);
}

// F4 .  HG .  LG .
// F3 E  .  HM .  LM
// F2 .  .  .  .  .
// F1 .  .  .  .  .
TEST(DAY11, FacilityOrderDFS1) {
    // Arrange
    const map<int, Floor> floors{
        {1, Floor({}, {})},
        {2, Floor({}, {})},
        {3, Floor({}, {"H", "L"})},
        {4, Floor({"H", "L"}, {})}
    };

    const auto initial_state = State(0, 3, floors);

    // Act
    const int actual = Facility::dfs_start(std::make_unique<State>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 1;
    EXPECT_EQ(actual, expected_steps);
}

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

    const auto initial_state = State(0, 1, floors);

    // Act
    const int actual = Facility::order(std::make_unique<State>(initial_state), 4);


    // Assert
    constexpr int expected_steps = 11;
    EXPECT_EQ(actual, expected_steps);
}

// TEST(DAY11, FacilityOrderDFS) {
//     // Arrange
//     const map<int, Floor> floors{
//         {1, Floor({}, {"H", "L"})},
//         {2, Floor({"H"}, {})},
//         {3, Floor({"L"}, {})},
//         {4, Floor({}, {})}
//     };
//
//     const auto initial_state = State(0, 1, floors);
//
//     // Act
//     const int actual = Facility::dfs_start(initial_state, 4);
//
//
//     // Assert
//     constexpr int expected_steps = 11;
//     EXPECT_EQ(actual, expected_steps);
// }

TEST(DAY11, StateCacheEquals) {
    // Arrange
    auto first_state = State(Elevator({"a"}, {"b"}), 1, 1,
                             {
                                 {1, Floor({"a"}, {"b"})},
                                 {2, Floor({"a"}, {"b"})}
                             });
    auto second_state = State(Elevator({"a"}, {"b"}), 1, 1,
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
