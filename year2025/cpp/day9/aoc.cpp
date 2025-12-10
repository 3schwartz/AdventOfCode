#include "aoc.h"
#include "../common/common.h"
#include <set>
#include <map>

using std::abs;
using std::map;
using std::max;
using std::min;
using std::pair;
using std::set;
using std::stoll;
using std::vector;

pair<int, int> parseCord(string &s)
{
    vector<string> parts = split(s, ',');
    return {std::stoi(parts[0]), std::stoi(parts[1])};
}

long long part1(vector<string> lines)
{
    long long area = 0;
    for (int i = 0; i < lines.size(); ++i)
    {
        pair<int, int> a = parseCord(lines[i]);
        for (int j = i + 1; j < lines.size(); ++j)
        {
            pair<int, int> b = parseCord(lines[j]);
            area = max(area, (long long)(abs(a.first - b.first) + 1) * (long long)(abs(a.second - b.second) + 1));
        }
    }
    return area;
}

vector<pair<int, int>> getCoords(vector<string> lines)
{
    vector<pair<int, int>> coords;
    for (const string &line : lines)
    {
        coords.push_back(parseCord(const_cast<string &>(line)));
    }
    return coords;
}

pair<int, int> x_y_max(vector<pair<int, int>> &coords)
{
    int x_max = 0;
    int y_max = 0;
    for (const pair<int, int> &p : coords)
    {
        if (p.first > x_max)
        {
            x_max = p.first;
        }
        if (p.second > y_max)
        {
            y_max = p.second;
        }
    }
    return {x_max, y_max};
}

set<pair<int, int>> border_points(vector<pair<int, int>> &coords)
{
    set<pair<int, int>> pts;
    for (int i = 0; i < coords.size(); ++i)
    {
        pair<int, int> a = coords[i % coords.size()];
        pair<int, int> b = coords[(i + 1) % coords.size()];
        for (int y = min(a.second, b.second); y <= max(a.second, b.second); ++y)
        {
            pts.insert({a.first, y});
            pts.insert({b.first, y});
        }
        for (int x = min(a.first, b.first); x <= max(a.first, b.first); ++x)
        {
            pts.insert({x, a.second});
            pts.insert({x, b.second});
        }
    }

    return pts;
}

/// @brief From
/// https://www.geeksforgeeks.org/dsa/how-to-check-if-a-given-point-lies-inside-a-polygon/
bool point_in_polygon(pair<int, int> point, vector<pair<int, int>> polygon)
{
    int num_vertices = polygon.size();
    int x = point.first, y = point.second;
    bool inside = false;

    pair<int, int> p1 = polygon[0], p2;

    for (int i = 1; i <= num_vertices; i++)
    {
        p2 = polygon[i % num_vertices];

        if (y > min(p1.second, p2.second))
        {
            if (y <= max(p1.second, p2.second))
            {
                if (x <= max(p1.first, p2.first))
                {

                    int x_intersection = (y - p1.second) * (p2.first - p1.first) / (p2.second - p1.second) + p1.first;
                    if (p1.first == p2.first || x <= x_intersection)
                    {
                        inside = !inside;
                    }
                }
            }
        }
        p1 = p2;
    }

    return inside;
}

set<pair<int, int>> interier_points(
    vector<pair<int, int>> &points,
    int x_max,
    int y_max)
{
    set<pair<int, int>> covered_pts;

    covered_pts.insert(points.begin(), points.end());

    for (int x = 0; x <= x_max; ++x)
    {
        for (int y = 0; y <= y_max; ++y)
        {
            pair<int, int> p = {x, y};
            if (point_in_polygon(p, vector<pair<int, int>>(points.begin(), points.end())))
            {
                covered_pts.insert(p);
            }
        }
    }
    return covered_pts;
}

long long largest_rectangle(
    vector<pair<int, int>> &coords,
    set<pair<int, int>> &pts)
{
    long long area = 0;

    for (int i = 0; i < coords.size(); ++i)
    {
        pair<int, int> a = coords[i];
        for (int j = i + 1; j < coords.size(); ++j)
        {
            pair<int, int> b = coords[j];
            if (a.first == 9 &&
                a.second == 5 &&
                b.first == 2 &&
                b.second == 3)
            {
                int debug = 1;
            }

            if (!(pts.contains({a.first, b.second}) &&
                  pts.contains({b.first, a.second})))
            {
                continue;
            }
            area = max(area, (long long)(abs(a.first - b.first) + 1) * (long long)(abs(a.second - b.second) + 1));
            if (area == 50)
            {
                int debug = 1;
            }
        }
    }
    return area;
}

long long part2(vector<string> lines)
{
    vector<pair<int, int>> coords = getCoords(lines);
    pair<int, int> max_coords = x_y_max(coords);
    set<pair<int, int>> interier_pts = border_points(coords);
    set<pair<int, int>> pts = interier_points(coords,
                                              max_coords.first,
                                              max_coords.second);
    long long area = largest_rectangle(coords, pts);
    return area;
}
