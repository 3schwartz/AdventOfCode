#include <iostream>
#include <bitset>
#include <map>
#include <utility>
#include <set>

constexpr int FAVORITE_NUMBER = 1352;

using std::cout;
using std::endl;
using std::bitset;
using std::map;
using std::pair;
using std::make_pair;
using std::set;
using std::queue;
using std::vector;

int calculate_is_open(int x, int y) {
    const int sum = x * x + 3 * x + 2 * x * y + y + y * y + FAVORITE_NUMBER;
    const bitset<32> bits(sum);
    return bits.count() % 2 == 0;
}

int is_open(map<pair<int, int>, bool> &map, int x, int y) {
    if (map.contains(make_pair(x, y))) {
        return map.at(make_pair(x, y));
    }
    bool is_open = calculate_is_open(x, y);
    map.insert({{x, y}, is_open});
    return is_open;
}

int main() {
    map<pair<int, int>, bool> cache;
    set<pair<int, int> > visited;
    queue<std::tuple<int, int, int> > queue;
    queue.emplace(1, 1, 0);

    vector<pair<int, int> > directions = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

    bool print_part_2 = true;
    while (!queue.empty()) {
        const auto [x, y, steps] = queue.front();
        queue.pop();
        if (!visited.insert({x, y}).second) {
            continue;
        }
        if (steps == 50 && print_part_2) {
            cout << "Part 2: " << visited.size() << endl;
            print_part_2 = false;
        }
        if (x == 31 && y == 39) {
            cout << "Part 1: " << steps << endl;
            return 0;
        }

        for (const auto &[dx, dy]: directions) {
            const int nx = x + dx;
            const int ny = y + dy;
            if (!is_open(cache, nx, ny)) {
                continue;
            }
            if (nx < 0 || ny < 0) {
                continue;
            }
            if (visited.contains({nx, ny})) {
                continue;
            }
            queue.emplace(nx, ny, steps + 1);
        }
    }

    return -1;
}
