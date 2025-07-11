#include "day17.h"
#include <iomanip>

using std::setw;
using std::setfill;
using std::hex;
using std::ostringstream;

#include <openssl/md5.h>

string hexadecimal(const unsigned char *digest, long size = MD5_DIGEST_LENGTH) {
    ostringstream oss;
    for (int i = 0; i < size; ++i) {
        oss << hex << setw(2) << setfill('0') << static_cast<int>(digest[i]);
    }
    return oss.str();
}

string md5_hash(const string &to_hash) {
    unsigned char digest[MD5_DIGEST_LENGTH];
    MD5(reinterpret_cast<const unsigned char *>(to_hash.c_str()), to_hash.length(), digest);
    return hexadecimal(digest, MD5_DIGEST_LENGTH);
}

State::State(string path, const pair<int, int> &position, int steps) : path(std::move(path)), position(position),
                                                                       steps(steps) {
}


[[nodiscard]] vector<State> State::gets_next_states(const string &hash) const {
    vector<State> next_states;
    for (int i = 0; i < 4; ++i) {
        if (const char h = hash[i]; !(h == 'b' || h == 'c' || h == 'd' || h == 'e' || h == 'f')) {
            continue;
        }
        const auto new_position = std::pair{position.first + moves[i].first, position.second + moves[i].second};
        if (new_position.first < 0 || new_position.second < 0 || new_position.first > 3 || new_position.second >
            3) {
            continue;
        }
        next_states.emplace_back(path + move_lookup[i], new_position, steps + 1);
    }
    return next_states;
}


pair<string, int> shortest_path(const string &salt, const bool stop_early) {
    set<pair<string, pair<int, int> > > visited;
    queue<State> q;
    q.push(State("", {0, 0}, 0));
    string o_path;
    int o_steps = 0;
    while (!q.empty()) {
        const auto state = q.front();
        q.pop();

        if (!visited.insert({state.path, state.position}).second) {
            continue;
        }
        if (state.position == std::pair{3, 3}) {
            o_path = state.path;
            o_steps = state.steps;
            if (stop_early) {
                break;
            }
            continue;
        }
        string hash = md5_hash(salt + state.path);
        const auto new_states = state.gets_next_states(hash);
        for (const auto &new_state: new_states) {
            q.push(new_state);
        }
    }
    return std::make_pair(o_path, o_steps);
}


