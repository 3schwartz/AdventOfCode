#include <set>
#include <sstream>
#include <utility>

#include "../common/common.h"

using std::set;
using std::pair;
using std::max;
using std::min;

set<pair<unsigned long, unsigned long> > get_edges(const vector<string> &data) {
    set<pair<unsigned long, unsigned long> > edges;
    for (const auto &line: data) {
        std::istringstream iss(line);
        char seperator;
        unsigned long from, to;
        iss >> from >> seperator >> to;
        edges.insert({from, to});
    }
    return edges;
}

unsigned long get_lowest_valued(const vector<string> &data) {
    const auto edges = get_edges(data);

    auto it = edges.begin();
    auto proposed = it->second + 1;
    while (++it != edges.end()) {
        if (proposed < it->first) {
            return proposed;
        }
        proposed = max(it->second + 1, proposed);
    }
    return proposed;
}

unsigned long get_all_ips(const vector<string> &data, unsigned long max_ip = 4294967295) {
    const auto edges = get_edges(data);

    auto it = edges.begin();
    unsigned long ips = it->first;
    auto last = it->second;
    while (++it != edges.end()) {
        if (last < it->first) {
            ips += it->first - last - 1;
        }
        last = max(it->second, last);
    }

    return ips + max_ip - last;
}

int main() {
    const auto data_test = read_lines("../../data/day20_data_test.txt");
    const unsigned long part_1_test = get_lowest_valued(data_test);
    if (part_1_test != 3) {
        std::cerr << part_1_test;
        exit(1);
    }

    const auto data = read_lines("../../data/day20_data.txt");
    const unsigned long part_1 = get_lowest_valued(data);

    cout << "Part 1: " << part_1 << endl;

    cout << "Part 2: " << get_all_ips(data) << endl;
    return 0;
}
