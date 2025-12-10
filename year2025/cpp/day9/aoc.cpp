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

pair<int, int> parseCord(const string &s)
{
    vector<string> parts = split(s, ',');
    return {std::stoi(parts[0]), std::stoi(parts[1])};
}

long long part1(const vector<string> &lines)
{
    long long area = 0;
    for (int i = 0; i < lines.size(); ++i)
    {
        const auto &a = parseCord(lines[i]);
        for (int j = i + 1; j < lines.size(); ++j)
        {
            const auto &b = parseCord(lines[j]);
            area = max(area, (long long)(abs(a.first - b.first) + 1) * (long long)(abs(a.second - b.second) + 1));
        }
    }
    return area;
}

vector<pair<int, int>> getCoords(const vector<string> &lines)
{
    vector<pair<int, int>> coords;
    coords.reserve(lines.size());
    for (const string &line : lines)
    {
        coords.push_back(parseCord(line));
    }
    return coords;
}

struct Line
{
    pair<int, int> start;
    pair<int, int> end;

    static vector<Line> getEdges(const vector<pair<int, int>> &points)
    {
        vector<Line> sides;
        int n = points.size();
        for (int i = 0; i < n; ++i)
        {
            sides.emplace_back(Line({points[i], points[(i + 1) % n]}));
        }
        return sides;
    }
};

bool pointIsInside(int x, int y, const vector<Line> &edges)
{
    int cross = 0;

    for (const Line &line : edges)
    {
        const auto &[a, b] = line;
        auto [xi, yi] = a;
        auto [xj, yj] = b;

        // if on edge
        if (min(xi, xj) <= x && x <= max(xi, xj) &&
            min(yi, yj) <= y && y <= max(yi, yj))
        {
            return true;
        }

        // how many edges from left to right (vertical edges)
        // are crossed.
        // if odd, point is inside
        if (xi == xj && min(yi, yj) <= y && y < max(yi, yj) && xi > x)
        {
            cross += 1;
        }
    }

    return cross % 2 == 1;
}

bool edgeIsIntersect(
    const Line &side,
    const Line &edge)
{
    const auto &[s_f, s_s] = side;
    const auto &[e_f, e_s] = edge;

    // check if both side and edge are vertical or horizontal
    if ((s_f.first == s_s.first && e_f.first == e_s.first) ||
        (s_f.second == s_s.second && e_f.second == e_s.second))
        return false;

    // side is vertical
    if (s_f.first == s_s.first)
    {
        if (min(e_f.first, e_s.first) < s_f.first && s_f.first < max(e_f.first, e_s.first) &&     // if edge x is on opposite side of line
            min(s_f.second, s_s.second) < e_f.second && e_f.second < max(s_f.second, s_s.second)) // and side has y_min and y_max on opposite sides of edge y
        {
            return true;
        }
    }
    // side is horizontal
    if (s_f.second == s_s.second)
    {
        if (min(e_f.second, e_s.second) < s_f.second && s_f.second < max(e_f.second, e_s.second) && // if edge y is on opposite side of line
            min(s_f.first, s_s.first) < e_f.first && e_f.first < max(s_f.first, s_s.first))         // and side has x_min and x_max on opposite sides of edge x
        {
            return true;
        }
    }

    return false;
}

bool isInsidePolygon(
    const pair<int, int> &a,
    const pair<int, int> &b,
    const vector<Line> &edges)

{
    auto [x1, y1] = a;
    auto [x2, y2] = b;
    int x_min = min(x1, x2);
    int x_max = max(x1, x2);
    int y_min = min(y1, y2);
    int y_max = max(y1, y2);

    vector<pair<int, int>> corners = {
        {x_min, y_min},
        {x_max, y_min},
        {x_max, y_max},
        {x_min, y_max}};

    for (const auto &[cx, cy] : corners)
    {
        if (!pointIsInside(cx, cy, edges))
        {
            return false;
        }
    }
    vector<Line> sides = {
        {{x_min, y_min}, {x_min, y_max}}, // left
        {{x_min, y_max}, {x_max, y_max}}, // bottom
        {{x_max, y_max}, {x_max, y_min}}, // right
        {{x_max, y_min}, {x_min, y_min}}, // top
    };
    for (const auto &side : sides)
    {
        for (const auto &edge : edges)
        {
            if (edgeIsIntersect(side, edge))
            {
                return false;
            }
        }
    }

    return true;
}

long long largestAreaWithin(const vector<pair<int, int>> &coords)
{
    long long area = 0;
    vector<Line> edges = Line::getEdges(coords);

    for (int i = 0; i < coords.size(); ++i)
    {
        const auto &a = coords[i];
        for (int j = i + 1; j < coords.size(); ++j)
        {
            const auto &b = coords[j];
            if (isInsidePolygon(a, b, edges))
            {
                area = max(area, (long long)(abs(a.first - b.first) + 1) *
                                     (long long)(abs(a.second - b.second) + 1));
            }
        }
    }
    return area;
}

long long part2(const vector<string> &lines)
{
    vector<pair<int, int>> coords = getCoords(lines);
    long long area = largestAreaWithin(coords);
    return area;
}
