#ifndef DAY11_V2_H
#define DAY11_V2_H

#include <map>
#include <utility>
#include <vector>
#include <set>
#include <string>
#include <sstream>

using std::map;
using std::set;
using std::vector;

using Item = std::pair<std::string, char>;
using FloorV2 = std::set<Item>;
using StateV2 = std::tuple<int, int, std::vector<FloorV2> >;

vector<set<std::pair<std::string, char> > > parse_states(const vector<std::string> &data);

std::vector<std::string> load_from_file(const std::string &filename);

bool is_done(const std::vector<FloorV2> &floors);

bool is_floor_safe(const FloorV2 &floor);

std::vector<std::vector<Item> > get_combinations(const FloorV2 &floor);

std::string get_floors_status(int elevator, const std::vector<FloorV2> &floors);

std::vector<StateV2> get_possible_steps(const StateV2 &state);

int run_steps(const std::vector<FloorV2> &floors);

int part1(const std::vector<std::string> &data);

int part2(const std::vector<std::string> &data);

#endif
