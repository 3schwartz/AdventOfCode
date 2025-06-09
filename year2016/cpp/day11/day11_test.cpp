#include <gtest/gtest.h>

#include <utility>
#include "day11.h"
using std::set;
using std::map;
using std::pair;
using std::string;
using std::vector;
using std::make_pair;

class Floor {
    int _level;
    set<string> _generators;
    set<string> _microchips;

public:
    explicit Floor(const int level) : _level(level) {
    }


    void add_generators(const set<string> &generators) {
        _generators.insert(generators.begin(), generators.end());
    }

    void add_microchips(const set<string> &microchips) {
        _microchips.insert(microchips.begin(), microchips.end());
    }


    // Check if valid: TODO

    vector<pair<set<string>, set<string> > > generate_pairs() const {
        vector<string> combined;
        combined.reserve(_generators.size() + _microchips.size());
        for (const string &generator: _generators) {
            combined.push_back(generator);
        }
        for (const string &microchip: _microchips) {
            combined.push_back(microchip);
        }

        vector<pair<set<string>, set<string> > > pairs;
        for (int i = 0; i < combined.size(); i++) {
            const string &first = combined[i];
            for (int j = i + 1; j < combined.size(); j++) {
                set<string> _g;
                set<string> _m;
                const string &second = combined[j];
                if (_generators.contains(first)) {
                    _g.insert(first);
                } else {
                    _m.insert(first);
                }
                if (_generators.contains(second)) {
                    _g.insert(second);
                } else {
                    _m.insert(second);
                }
                pairs.emplace_back(_g, _m);
            }
            set<string> _g;
            set<string> _m;
            if (_generators.contains(first)) {
                _g.insert(first);
            } else {
                _m.insert(first);
            }
            pairs.emplace_back(_g, _m);
        }
        return pairs;
    }
};

TEST(DAY11, GeneratePairs) {
    // Arrange
    Floor floor(1);
    floor.add_generators({"a", "b"});
    floor.add_microchips({"c", "d"});

    // Act
    vector<pair<set<string>, set<string> > > pairs = floor.generate_pairs();

    // Assert
    vector<pair<set<string>, set<string> > > expected{
        {{"a", "b"}, {}},
        {{"a"}, {"c"}},
        {{"a"}, {"d"}},
        {{"a"}, {}},
        {{"b"}, {"c"}},
        {{"b"}, {"d"}},
        {{"b"}, {}},
        {{}, {"c", "d"}},
        {{}, {"c"}},
        {{}, {"d"}}
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
