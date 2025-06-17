#ifndef DAY11_H
#define DAY11_H
#include <queue>
#include <utility>
#include <map>
#include <ranges>
#include <set>

using std::set;
using std::map;
using std::pair;
using std::string;
using std::vector;
using std::make_pair;
using std::tuple;
using std::move;
using std::queue;

struct Elevator {
    std::set<std::string> _generators;
    std::set<std::string> _microchips;

    Elevator(std::set<std::string> generators, std::set<std::string> microchips);

    auto operator<=>(const Elevator &other) const = default;

    std::pair<std::set<std::string>, std::set<std::string> > generate_cache();
};

class Floor {
    set<string> _generators;
    set<string> _microchips;

public:
    Floor(std::set<std::string> generators, std::set<std::string> microchips);

    auto operator<=>(const Floor &other) const = default;

    void add_generators(const std::set<std::string> &generators);

    void add_microchips(const std::set<std::string> &microchips);

    vector<std::pair<Elevator, Floor> > generate_elevator();

    vector<Elevator> generate_pairs() const;

    std::pair<std::set<std::string>, std::set<std::string> > generate_cache();

    [[nodiscard]] bool is_valid() const;

    [[nodiscard]] bool is_empty() const;
};

using StateCache = pair<int, std::map<int, pair<set<string>, set<string> > > >;

class State {
    int _steps;
    int _level;
    map<int, Floor> _floors;

public:
    State(int steps, int level, std::map<int, Floor> floors);

    State(const Elevator &elevator, int steps, int level, std::map<int, Floor> floors);

    auto operator<=>(const State &other) const = default;

    [[nodiscard]] int steps() const;

    void hydrate_from_elevator(const Elevator &elevator);

    [[nodiscard]] bool is_level_valid() const;

    bool all_on_level(int level);

    std::vector<State> next_states(int max_level);

    StateCache generate_cache();
};

class Facility {
public:
    static int dfs_start(State state, int final_level);

    static int order(State initial_state, int final_level);

private:
    static int dfs_iterate(
        State state,
        map<StateCache, int> &optimal,
        map<StateCache, int> &best_seen,
        int final_level,
        int &global_min_steps);
};


#endif
