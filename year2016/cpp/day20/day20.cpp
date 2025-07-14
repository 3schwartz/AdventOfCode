#include <set>
#include <sstream>
#include <utility>

#include "../common/common.h"

int get_lowest_valued(const vector<string> &data) {
    std::set<std::pair<int, int> > edges;
    for (const auto &line: data) {
        std::istringstream iss(line);
        char seperator;
        int from, to;
        iss >> from >> seperator >> to;

        bool insert = true;
        for (auto &[fst, snd]: edges) {
            // within from below or above
            if (fst <= to + 1 && to <= snd || fst <= from && from - 1 <= snd) {
                int new_fst = std::min(fst, from);
                int new_snd = std::max(snd, to);

                edges.erase({fst, snd});
                edges.insert({new_fst, new_snd});

                insert = false;
                break;
            }
            // new within
            if (fst <= from && to <= snd) {
                insert = false;
                break;
            }
            // old within
            if (from <= fst && snd <= to) {
                edges.erase({fst, snd});
                edges.insert({from, to});
                insert = false;
                break;
            }
        }

        if (insert) {
            edges.insert({from, to});
        }
    }

    auto it = edges.begin();
    auto proposed = it->second + 1;
    while (++it != edges.end()) {
        if (proposed < it->first) {
            return proposed;
        }
        proposed = std::max(it->second + 1, proposed);
    }
    return proposed;
}

int main() {
    const auto data_test = read_lines("../../data/day20_data_test.txt");
    const int part_1_test = get_lowest_valued(data_test);
    if (part_1_test != 3) {
        std::cerr << part_1_test;
        exit(1);
    }

    const auto data = read_lines("../../data/day20_data.txt");
    const int part_1 = get_lowest_valued(data);

    cout << "Part 1: " << part_1 << endl;

    return 0;
}
