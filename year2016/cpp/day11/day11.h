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


class IState {
public :
    virtual ~IState() = default;

    auto operator<=>(const IState &) const = default;

    virtual string generate_cache() = 0;

    virtual bool all_on_level(int final_level) = 0;

    virtual int steps() = 0;

    virtual vector<std::unique_ptr<IState> > next_states(int final_level) = 0;
};

enum HardwareType {
    GENERATOR,
    MICROCHIP,
};

using ElevatorOption = set<pair<string, HardwareType> >;

class FloorSimple {
    set<pair<string, HardwareType> > _hardware;

public:
    FloorSimple(set<pair<string, HardwareType> > hardware);

    vector<pair<ElevatorOption, FloorSimple> > generate_elevator();

    [[nodiscard]] vector<ElevatorOption> generate_pairs() const;

    [[nodiscard]] bool is_empty() const;

    [[nodiscard]] bool is_valid() const;

    auto operator<=>(const FloorSimple &other) const = default;

    void add_hardware(const ElevatorOption &set);

    [[nodiscard]] string generate_cache() const;
};

using StateCache = string;

class StateSimple final : public IState {
    int _steps;
    int _level;
    map<int, FloorSimple> _floors;

public:
    StateSimple(int steps, int level, std::map<int, FloorSimple> floors);

    StateSimple(const ElevatorOption &elevator, int steps, int level, std::map<int, FloorSimple> floors);

    auto operator<=>(const StateSimple &other) const = default;

    int steps() override;

    void hydrate_from_elevator(const ElevatorOption &elevator);

    [[nodiscard]] bool is_level_valid() const;

    bool all_on_level(int level) override;

    std::vector<std::unique_ptr<IState> > next_states(int max_level) override;

    StateCache generate_cache() override;
};

struct Elevator {
    std::set<std::string> _generators;
    std::set<std::string> _microchips;

    Elevator(std::set<std::string> generators, std::set<std::string> microchips);

    auto operator<=>(const Elevator &other) const = default;
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

    string generate_cache() const;

    [[nodiscard]] bool is_valid() const;

    [[nodiscard]] bool is_empty() const;
};


class State final : public IState {
    int _steps;
    int _level;
    map<int, Floor> _floors;

public:
    State(int steps, int level, std::map<int, Floor> floors);

    State(const Elevator &elevator, int steps, int level, std::map<int, Floor> floors);

    auto operator<=>(const State &other) const = default;

    int steps() override;

    void hydrate_from_elevator(const Elevator &elevator);

    [[nodiscard]] bool is_level_valid() const;

    bool all_on_level(int level) override;

    std::vector<std::unique_ptr<IState> > next_states(int max_level) override;

    StateCache generate_cache() override;
};

class Facility {
public:
    static int dfs_start(std::unique_ptr<IState> state, int final_level);

    static int order(std::unique_ptr<IState> initial_state, int final_level);

private:
    static int dfs_iterate(
        const std::unique_ptr<IState> &state,
        map<StateCache, int> &optimal,
        map<StateCache, int> &best_seen,
        int final_level,
        int &global_min_steps);
};


#endif
