#include <iostream>
#include <string>
#include "day14.h"
#include <map>
#include <iomanip>
#include <sstream>
#define OPENSSL_API_COMPAT 0x10100000L
#include <set>
#include <openssl/md5.h>

using std::setw;
using std::setfill;
using std::hex;
using std::ostringstream;
using std::set;
using std::pair;
using std::map;

string hexadecimal(const unsigned char *digest, long size = MD5_DIGEST_LENGTH) {
    ostringstream oss;
    for (int i = 0; i < size; ++i) {
        oss << hex << setw(2) << setfill('0') << static_cast<int>(digest[i]);
    }
    return oss.str();
}

string md5_hash(const int index, const string &salt) {
    const string to_hash = salt + std::to_string(index);
    unsigned char digest[MD5_DIGEST_LENGTH];
    MD5(reinterpret_cast<const unsigned char *>(to_hash.c_str()), to_hash.length(), digest);
    return hexadecimal(digest, MD5_DIGEST_LENGTH);
}

pair<set<char>, set<char> > find_duplicates(const string &hexadecimal_hash) {
    set<char> fives;
    set<char> threes;

    int same = 0;
    bool has_found_triplet = false;
    for (size_t i = 1; i < hexadecimal_hash.size(); ++i) {
        if (hexadecimal_hash[i - 1] != hexadecimal_hash[i]) {
            same = 0;
            continue;
        }
        same++;
        if (same == 2 && !has_found_triplet) {
            threes.insert(hexadecimal_hash[i]);
            has_found_triplet = true;
        }
        if (same == 4) {
            fives.insert(hexadecimal_hash[i]);
        }
    }

    return make_pair(fives, threes);
}

set<int> below_which_has_three(
    const map<char, set<int> > &cache,
    const char five,
    const int five_index,
    const int limit) {
    set<int> belows;
    if (!cache.contains(five)) {
        return belows;
    }
    for (const auto &indexes = cache.at(five); const int index: indexes) {
        if (index < five_index && index >= five_index - limit) {
            belows.insert(index);
        }
    }
    return belows;
}

void update_three_cache(
    map<char, set<int> > &cache,
    const set<char> &threes,
    int index) {
    for (const char three: threes) {
        if (!cache.contains(three)) {
            cache.insert({{three, {index}}});
        } else {
            cache.at(three).insert(index);
        }
    }
}

set<int> keys(
    map<char, set<int> > &cache,
    const int index,
    const string &salt,
    const int limit
) {
    const string hash = md5_hash(index, salt);

    auto [fives, threes] = find_duplicates(hash);

    update_three_cache(cache, threes, index);

    set<int> belows;
    for (const char five: fives) {
        set<int> belows_five = below_which_has_three(cache, five, index, limit);
        belows.insert(belows_five.begin(), belows_five.end());
    }

    return belows;
}

int find_key(
    const int key_number,
    const string &salt,
    const int limit
) {
    map<char, set<int> > cache;
    int index = 0;
    set<int> indexes;
    while (indexes.size() < key_number) {
        set<int> keys_below = keys(cache, index, salt, limit);
        indexes.insert(keys_below.begin(), keys_below.end());
        index++;
    }

    auto it = indexes.begin();
    std::advance(it, key_number - 1);
    return *it;
}
