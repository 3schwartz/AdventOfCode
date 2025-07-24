#include <map>
#include <set>
#include <utility>

#include "../common/common.h"
using std::map;
using std::set;
using std::pair;
using std::make_pair;
using std::tuple;
using std::queue;

struct MinStepState {
    int x;
    int y;
    int steps;
    set<pair<int, int> > visited;
    set<int> numbers;

    MinStepState(int x, int y, int steps, set<pair<int, int> > visited, set<int> numbers) {
        this->x = x;
        this->y = y;
        this->steps = steps;
        this->visited = std::move(visited);
        this->numbers = std::move(numbers);
    }

    tuple<int, int, int, set<pair<int, int> >, set<int> > cache() {
        return tuple(x, y, steps, visited, numbers);
    }
};

using MinStepCache = tuple<int, int, int, set<pair<int, int> >, set<int> >;

constexpr std::array<pair<int, int>, 4> moves = {
    {{0, -1}, {0, 1}, {-1, 0}, {1, 0}}
};

struct InitialState {
    int x_start;
    int y_start;
    map<pair<int, int>, int> numbers;
    set<pair<int, int> > points;

    explicit InitialState(const vector<string> &data) {
        for (int y = 0; y < data.size(); y++) {
            auto &line = data[y];
            for (int x = 0; x < line.size(); x++) {
                auto &c = line[x];
                if (c == '#') {
                    continue;
                }
                points.insert(make_pair(x, y));
                if (c == '0') {
                    x_start = x;
                    y_start = y;
                }
                if (c >= '1' && c <= '9') {
                    numbers.insert(make_pair(make_pair(x, y), c - '0'));
                }
            }
        }
    }

    int min_steps() const {
        queue<MinStepState> q;
        q.emplace(x_start, y_start, 0, set<pair<int, int> >{{x_start, y_start}}, set<int>{});
        set<MinStepCache> cache;

        while (!q.empty()) {
            auto state = q.front();
            q.pop();
            if (!cache.insert(state.cache()).second) {
                continue;
            }
            if (state.numbers.size() == numbers.size()) {
                return state.steps;
            }

            for (const auto &[dx, dy]: moves) {
                const int nx = state.x + dx;
                const int ny = state.y + dy;
                if (!points.contains({nx, ny})) {
                    continue;
                }
                auto n_numbers = state.numbers;
                auto n_visited = state.visited;
                n_visited.insert({nx, ny});
                if (numbers.contains({nx, ny})) {
                    n_numbers.insert(numbers.at({nx, ny}));
                }
                q.emplace(nx, ny, state.steps + 1, std::move(n_visited), std::move(n_numbers));
            }
        };

        return -1;
    }
};


int main() {
    const auto data_test = read_lines("../../data/day24_data_test.txt");
    const auto state_test = InitialState{data_test};
    const int min_steps_test = state_test.min_steps();
    if (min_steps_test != 14) {
        std::cerr << min_steps_test;
        exit(1);
    }
    const auto data = read_lines("../../data/day24_data.txt");
    const auto state = InitialState{data};
    const int min_steps = state.min_steps();
    cout << "Part 1: " << min_steps_test << endl;
}
