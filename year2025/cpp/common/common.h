#ifndef COMMON_H
#define COMMON_H
#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

vector<string> read_lines(const string &filename);

int mod(int a, int b);
#endif // COMMON_H
