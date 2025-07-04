#ifndef COMMON_H
#define COMMON_H
#include <iostream>
#include <vector>

using std::vector;
using std::string;
using std::cout;
using std::endl;

vector<string> read_lines(const string &filename);

int mod(int a, int b);
#endif // COMMON_H
