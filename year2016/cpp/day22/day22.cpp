#include <map>
#include <sstream>
#include <set>

#include "../common/common.h"

using std::string;
using std::stoi;
using std::pair;
using std::vector;
using std::queue;
using std::map;
using std::set;
using std::make_pair;

vector<string> split_by(const string &s, char delim) {
    vector<string> elems;
    std::stringstream ss(s);
    string t;
    while (getline(ss, t, delim)) {
        elems.push_back(t);
    }
    return elems;
}

struct File {
    int x;
    int y;
    int size;
    int used;
    int avail;
    int use_p;

    explicit File(const string &f) {
        vector<string> tokens;
        std::istringstream iss(f);
        string token;
        while (iss >> token) {
            tokens.push_back(token);
        }
        if (tokens.size() != 5) {
            throw std::runtime_error("Invalid file format " + f);
        }
        const string coord_t = tokens[0].substr(15);
        const auto coords = split_by(coord_t, '-');
        if (coords.size() != 2) {
            throw std::runtime_error("Invalid coord: " + f);
        }
        x = stoi(coords[0].substr(1));
        y = stoi(coords[1].substr(1));
        size = stoi(tokens[1].substr(0, tokens[1].size() - 1));
        used = stoi(tokens[2].substr(0, tokens[2].size() - 1));
        avail = stoi(tokens[3].substr(0, tokens[3].size() - 1));
        use_p = stoi(tokens[4].substr(0, tokens[4].size() - 1));
    }
};

vector<File> parse_files(const vector<string> &lines) {
    vector<File> files;
    files.reserve(lines.size());
    for (int i = 2; i < lines.size(); i++) {
        files.emplace_back(lines[i]);
    }
    return files;
}

int valid_pairs(const vector<File> &files) {
    int valid_pairs = 0;
    for (const auto &file_i: files) {
        if (file_i.used == 0) {
            continue;
        }
        for (const auto &file_j: files) {
            if (file_i.x == file_j.x && file_i.y == file_j.y) {
                continue;
            }
            if (file_i.used > file_j.avail) {
                continue;
            }
            valid_pairs++;
        }
    }
    return valid_pairs;
}

/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  G
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  #  #  #  #  #  #
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
/// .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  _  .
///
/// One can see that
/// - there is only 1 with 0 used
/// - one needs to go left around big files, since these can never fit into the free space
void print_overview(const vector<File> &files) {
    int x_max = 0;
    int y_max = 0;
    map<pair<int, int>, pair<int, int> > files_map;
    for (const auto &file: files) {
        x_max = std::max(x_max, file.x);
        y_max = std::max(y_max, file.y);
        files_map[make_pair(file.x, file.y)] = make_pair(file.size, file.used);
    }

    for (int y = 0; y <= y_max; y++) {
        for (int x = 0; x <= x_max; x++) {
            const auto &[size, used] = files_map[make_pair(x, y)];
            if (size > 100) {
                cout << " # ";
            } else if (used == 0) {
                cout << " _ ";
            } else if (y == 0 && x == x_max) {
                cout << " G ";
            } else {
                cout << " . ";
            }
        }
        cout << endl;
    }
}

/// Go left until free of big files
/// #_x_min - _x - 1 +
///
/// Move to y = 0 from the empty position
/// _y +
///
/// Move to just before the Goal
/// (x_max - 1) - (#_x_min - 1) +
///
/// The Goal needs to move from the end to the start
/// x_max +
///
/// For every step left, the empty slot needs to "go around," which costs 4 steps
/// (x_max - 1) * 4
int min_steps_calculated(const vector<File> &files) {
    int x_max = INT_MIN;
    int big_x_min = INT_MAX;
    int empty_x = INT_MIN;
    int empty_y = INT_MIN;
    for (const auto &file: files) {
        if (file.size > 100) {
            big_x_min = std::min(big_x_min, file.x);
        }
        if (file.used == 0) {
            empty_x = file.x;
            empty_y = file.y;
        }
        x_max = std::max(x_max, file.x);
    }
    int result = empty_x - big_x_min + 1;
    result += empty_y;
    result += (x_max - 1) - (big_x_min - 1);
    result += x_max;
    result += (x_max - 1) * 4;;
    return result;
}

struct [[deprecated("Use min_steps_calculated instead")]] FileState {
    int used;
    int size;

    [[nodiscard]] pair<int, int> cache() const {
        return make_pair(used, size);
    }
};

[[deprecated("Use min_steps_calculated instead")]] constexpr std::array<pair<int, int>, 4> moves = {
    {{0, -1}, {0, 1}, {-1, 0}, {1, 0}}
};

using StateCache [[deprecated("Use min_steps_calculated instead")]] = pair<pair<int, int>, map<pair<int, int>, pair<int,
    int> > >;


struct [[deprecated("Use min_steps_calculated instead")]] State {
    int steps;
    int x_goal;
    int y_goal;
    map<pair<int, int>, FileState> files;
    set<pair<int, int> > zeros;

    explicit State(const vector<File> &files) {
        int x_max = 0;
        for (const auto &file: files) {
            x_max = std::max(x_max, file.x);
            this->files.insert({{file.x, file.y}, {file.used, file.size}});
            if (file.used == 0) {
                zeros.insert({file.x, file.y});
            }
        }
        x_goal = x_max;
        y_goal = 0;
        steps = 0;
    }

    [[nodiscard]] StateCache cache() const {
        map<pair<int, int>, pair<int, int> > cache_map;
        for (auto &[k, v]: files) {
            cache_map.insert({k, v.cache()});
        }
        return make_pair(make_pair(x_goal, y_goal), cache_map);
    }


    int min_steps() {
        queue<State> q;
        q.emplace(*this);
        set<StateCache> visited;

        while (!q.empty()) {
            const auto state = q.front();
            q.pop();
            if (!visited.insert(state.cache()).second) {
                continue;
            }
            if (state.x_goal == 0 && state.y_goal == 0) {
                return state.steps;
            }
            for (const auto &[x, y]: state.zeros) {
                const auto &file_state = state.files.at({x, y});

                for (const auto &[dx, dy]: moves) {
                    const int nx = x + dx;
                    const int ny = y + dy;
                    if (!state.files.contains({nx, ny})) {
                        continue;
                    }
                    const auto &neighbour = state.files.at({nx, ny});
                    if (file_state.size < neighbour.used) {
                        continue;
                    }
                    auto state_copy = state;

                    state_copy.files.at({x, y}).used = neighbour.used;
                    state_copy.files.at({nx, ny}).used = 0;
                    if (nx == state.x_goal && ny == state.y_goal) {
                        state_copy.x_goal = x;
                        state_copy.y_goal = y;
                    }
                    state_copy.zeros.erase({x, y});
                    state_copy.zeros.insert({nx, ny});
                    state_copy.steps++;
                    q.emplace(state_copy);
                }
            }
        }

        return -1;
    }
};


int main() {
    const auto data = read_lines("../../data/day22_data.txt");
    const auto files = parse_files(data);
    const int pairs = valid_pairs(files);

    cout << "Part 1: " << pairs << endl;

    const auto data_test = read_lines("../../data/day22_data_test.txt");
    const auto files_test = parse_files(data_test);
    State state_test(files_test);
    const int min_steps_test = state_test.min_steps();
    if (min_steps_test != 7) {
        std::cerr << min_steps_test;
        exit(1);
    }

    print_overview(files);
    cout << "Part 2: " << min_steps_calculated(files) << endl;
}
