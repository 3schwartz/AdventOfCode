//
// Created by Søren Schwartz on 04/07/2025.
//

#ifndef DAY17_H
#define DAY17_H

#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <map>

using std::string;
using std::move;
using std::set;
using std::pair;
using std::queue;
using std::vector;
using std::array;
using std::map;


struct State {
    string path;
    pair<int, int> position;
    int steps;
    static constexpr array<pair<int, int>, 4> moves = {{{0, -1}, {0, 1}, {-1, 0}, {1, 0}}};
    static constexpr array<char, 4> move_lookup = {'U', 'D', 'L', 'R'};

    State(string path, const pair<int, int> &position, int steps);

    [[nodiscard]] vector<State> gets_next_states(const string &hash) const;
};

pair<string, int> shortest_path(const string &salt, bool stop_early);

#endif //DAY17_H
