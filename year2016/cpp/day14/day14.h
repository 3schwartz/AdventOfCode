#ifndef DAY14_H
#define DAY14_H
#include <string>

using std::string;

string md5_hash_of_index(int index, const string &salt, int key_stretches);

int find_key(int key_number, const string &salt, int limit, int key_stretches = 0);

#endif //DAY14_H
