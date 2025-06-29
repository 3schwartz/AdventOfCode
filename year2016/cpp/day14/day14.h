#ifndef DAY14_H
#define DAY14_H
#include <string>

using std::string;

string md5_hash(int index, const string &salt);

int find_key(int key_number, const string &salt, int limit);

#endif //DAY14_H
