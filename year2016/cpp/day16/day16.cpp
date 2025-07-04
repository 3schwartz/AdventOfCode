#include "../common/common.h"

using std::string;

const string input = "10010000000110000";

string expand(const string &a, const int disk_length = 272) {
    if (a.size() >= disk_length) {
        return a.substr(0, disk_length);
    }
    string b = a;
    std::reverse(b.begin(), b.end());
    for (auto &c: b) {
        c = c == '1' ? '0' : '1';
    }
    const string new_a = a + '0' + b;
    return expand(new_a, disk_length);
}

string checksum(const string &a) {
    if (a.size() % 2 == 1) {
        return a;
    }
    string n(a.size() / 2, '0');
    for (int i = 0; i < a.size(); i += 2) {
        if (a[i] == a[i + 1]) {
            n[i / 2] = '1';
        }
    }
    return checksum(n);
}

string expand_and_checksum(const string &a, const int disk_length = 272) {
    return checksum(expand(a, disk_length));
}

int main() {
    const string dt1 = expand("111100001010", 14);
    cout << "Test Disk 1: " << dt1 << endl;

    const string tcs1 = checksum("110010110100");
    cout << "Test Checksum 1: " << tcs1 << endl;

    const string dt2 = expand_and_checksum("10000", 20);
    cout << "Test Disk 2: " << dt2 << endl;

    const string part_1 = expand_and_checksum(input);
    cout << "Part 1: " << part_1 << endl;

    const string part_2 = expand_and_checksum(input, 35651584);
    cout << "Part 2: " << part_2 << endl;

    return 0;
}
