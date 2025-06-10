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

struct State {
    // TODO
};

struct Move {
    set<string> _generators;
    set<string> _microchips;

    auto operator<=>(const Move &other) const = default;
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

    // Check if valid: TODO

    vector<pair<Move, Floor> > generate_move() {
        const auto pairs = generate_pairs();
        vector<pair<Move, Floor> > moves;
        for (const auto &[move_generator, move_microchip]: pairs) {
            auto generators = move_generator;
            auto microchips = move_microchip;

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
            moves.emplace_back(Move{generators, microchips}, Floor{_level, new_generators, new_microchips});
        }
        return moves;
    }

    vector<Move> generate_pairs() const {
        vector<string> combined;
        auto generator_size = _generators.size();
        auto microchip_size = _microchips.size();
        combined.reserve(generator_size + microchip_size);
        for (const string &generator: _generators) combined.emplace_back(generator);
        for (const string &microchip: _microchips) combined.emplace_back(microchip);

        vector<Move> pairs;
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
};

TEST(DAY11, GenerateMoves) {
    // Arrange
    Floor floor(1);
    floor.add_generators({"a", "b"});
    floor.add_microchips({"a", "b"});

    // Act
    vector<pair<Move, Floor> > moves = floor.generate_move();

    // Assert
    vector<Move> moves_;
    moves_.reserve(moves.size());
    for (const auto &move: moves | std::views::keys) {
        moves_.push_back(move);
    }

    const vector<Move> expected{
        {{"a"}, {"a"}},
        {{"b"}, {"b"}},
        {{}, {"a", "b"}},
        {{}, {"a"}},
        {{}, {"b"}}
    };

    EXPECT_EQ(moves.size(), 5);
    EXPECT_EQ(
        moves_,
        expected
    );
    EXPECT_EQ(moves[0].second, Floor(1, {"b"}, {"b"}));
};

TEST(DAY11, GeneratePairs) {
    // Arrange
    Floor floor(1);
    floor.add_generators({"a", "b"});
    floor.add_microchips({"a", "b"});

    // Act
    const vector<Move> pairs = floor.generate_pairs();

    // Assert
    const vector<Move> expected{
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


class Elevator {
    int _level;
    set<string> _generators;
    set<string> _microchips;

public:
    Elevator(const int level, set<string> generators,
             set<string> microchips): _level(level),
                                      _generators(std::move(generators)),
                                      _microchips(std::move(microchips)) {
    }


    // Check if valid
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
