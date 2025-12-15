#ifndef COMMON_H
#define COMMON_H
#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

vector<string> read_lines(const string &filename);
vector<string> split(const string &s, char delimiter);
vector<string> split(const std::string &s, const std::string &delimiter);
int mod(int a, int b);
string trim(const std::string &s);

#endif // COMMON_H
