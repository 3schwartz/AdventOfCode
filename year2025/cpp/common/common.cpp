#include "common.h"

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

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

vector<std::string> split(const std::string &s, const std::string &delimiter)
{
    std::vector<std::string> parts;
    size_t start = 0;
    size_t end;

    while ((end = s.find(delimiter, start)) != std::string::npos)
    {
        parts.push_back(s.substr(start, end - start));
        start = end + delimiter.length();
    }

    parts.push_back(s.substr(start));
    return parts;
}

string trim(const std::string &s)
{
    const std::string whitespace = " \t\n\r\f\v";

    size_t start = s.find_first_not_of(whitespace);
    if (start == std::string::npos)
        return ""; // string is all whitespace

    size_t end = s.find_last_not_of(whitespace);
    return s.substr(start, end - start + 1);
}
