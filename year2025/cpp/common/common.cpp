#include "common.h"

#include <iostream>
#include <fstream>
#include <sstream>

using std::ifstream;
using std::stringstream;

vector<string> read_lines(const string &filename)
{
    vector<string> lines;
    ifstream file(filename);

    if (!file.is_open())
    {
        std::cerr << "Error: Unable to open file" << filename << std::endl;
        return lines;
    }

    string line;
    while (std::getline(file, line))
    {
        lines.push_back(line);
    }
    return lines;
}

int mod(const int a, const int b)
{
    return (a % b + b) % b;
}

vector<string> split(const string &s, char delimiter)
{
    vector<string> parts;
    stringstream ss(s);
    string item;

    while (std::getline(ss, item, delimiter))
    {
        parts.push_back(item);
    }
    return parts;
}
