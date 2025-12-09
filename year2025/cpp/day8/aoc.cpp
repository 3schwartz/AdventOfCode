#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>

using std::make_tuple;
using std::map;
using std::pair;
using std::set;
using std::stoll;
using std::tuple;
using std::vector;

struct DistancePair
{
    tuple<int, int, int> point1;
    tuple<int, int, int> point2;

    double euclidean_distance()
    {
        auto [x1, y1, z1] = point1;
        auto [x2, y2, z2] = point2;

        return std::sqrt(
            std::pow(x1 - x2, 2) +
            std::pow(y1 - y2, 2) +
            std::pow(z1 - z2, 2));
    }
};

bool should_insert(
    vector<set<tuple<int, int, int>>> &circuits,
    const DistancePair &pair)
{
    for (auto &circuit : circuits)
    {
        if (circuit.contains(pair.point1) && circuit.contains(pair.point2))
        {
            return false;
        }
        if (circuit.contains(pair.point1) || circuit.contains(pair.point2))
        {
            return true;
        }
    }
    return true;
}

void update_circuits(vector<set<tuple<int, int, int>>> &circuits)
{
    bool merged = true;
    while (merged)
    {
        merged = false;
        for (int i = 0; i < circuits.size(); i++)
        {
            for (int j = i + 1; j < circuits.size(); j++)
            {
                set<tuple<int, int, int>> intersection;
                std::set_intersection(
                    circuits[i].begin(), circuits[i].end(),
                    circuits[j].begin(), circuits[j].end(),
                    std::inserter(intersection, intersection.begin()));
                if (!intersection.empty())
                {
                    circuits[i].insert(circuits[j].begin(), circuits[j].end());
                    circuits.erase(circuits.begin() + j);
                    merged = true;
                    break;
                }
            }
            if (merged)
            {
                break;
            }
        }
    }
}

vector<tuple<int, int, int>> make_points(const vector<string> &lines)
{
    vector<tuple<int, int, int>> points;
    for (const string &line : lines)
    {
        vector<string> parts = split(line, ',');
        if (parts.size() != 3)
        {
            throw std::invalid_argument("Invalid line format: " + line);
        }
        points.push_back(make_tuple(stoi(parts[0]), stoi(parts[1]), stoi(parts[2])));
    }
    return points;
}

map<double, vector<DistancePair>> make_distance_map(const vector<tuple<int, int, int>> &points)
{
    map<double, vector<DistancePair>> distance_map;

    for (int i = 0; i < points.size(); i++)
    {
        auto [x1, y1, z1] = points[i];
        for (int j = i + 1; j < points.size(); j++)
        {
            auto [x2, y2, z2] = points[j];
            auto distance_pair = DistancePair{points[i], points[j]};
            double dist = distance_pair.euclidean_distance();
            distance_map[dist].push_back(distance_pair);
        }
    }

    return distance_map;
}

long long part1(vector<string> lines, int count_threshold)
{
    vector<tuple<int, int, int>> points = make_points(lines);

    map<double, vector<DistancePair>> distance_map = make_distance_map(points);

    vector<set<tuple<int, int, int>>> circuits;
    int count = 0;

    for (const auto &[dist, dist_pairs] : distance_map)
    {
        if (count == count_threshold)
        {
            break;
        }
        for (const auto &pair : dist_pairs)
        {
            if (count == count_threshold)
            {
                break;
            }
            set<tuple<int, int, int>> new_circuit;
            new_circuit.insert(pair.point1);
            new_circuit.insert(pair.point2);
            circuits.push_back(new_circuit);
            update_circuits(circuits);
            count++;
        }
    }

    std::ranges::sort(circuits,
                      [](auto &a, auto &b)
                      { return a.size() > b.size(); });

    long long total = 1;
    for (int i = 0; i < 3 && i < circuits.size(); i++)
    {
        total *= circuits[i].size();
    }

    return total;
}

long long part2(vector<string> lines)
{
    vector<tuple<int, int, int>> points = make_points(lines);

    map<double, vector<DistancePair>> distance_map = make_distance_map(points);

    vector<set<tuple<int, int, int>>> circuits;
    int connections = 0;
    for (const auto &[dist, dist_pairs] : distance_map)
    {
        for (const auto &pair : dist_pairs)
        {
            bool inserted = should_insert(circuits, pair);
            if (!inserted)
            {
                continue;
            }
            connections++;
            if (connections == lines.size() - 1)
            {
                return (long long)std::get<0>(pair.point1) * (long long)std::get<0>(pair.point2);
            }
            set<tuple<int, int, int>> new_circuit;
            new_circuit.insert(pair.point1);
            new_circuit.insert(pair.point2);
            circuits.push_back(new_circuit);
            update_circuits(circuits);
        }
    }

    return -1;
}
