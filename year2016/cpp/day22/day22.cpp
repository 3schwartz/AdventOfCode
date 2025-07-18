#include <sstream>

#include "../common/common.h"

using std::string;
using std::stoi;

vector<string> split_by(const string &s, char delim) {
    vector<string> elems;
    std::stringstream ss(s);
    string t;
    while (getline(ss, t, delim)) {
        elems.push_back(t);
    }
    return elems;
}

struct File {
    int x;
    int y;
    int size;
    int used;
    int avail;
    int use_p;

    explicit File(const string &f) {
        vector<string> tokens;
        std::istringstream iss(f);
        string token;
        while (iss >> token) {
            tokens.push_back(token);
        }
        if (tokens.size() != 5) {
            throw std::runtime_error("Invalid file format " + f);
        }
        const string coord_t = tokens[0].substr(15);
        const auto coords = split_by(coord_t, '-');
        if (coords.size() != 2) {
            throw std::runtime_error("Invalid coord: " + f);
        }
        x = stoi(coords[0].substr(1));
        y = stoi(coords[1].substr(1));
        size = stoi(tokens[1].substr(0, tokens[1].size() - 1));
        used = stoi(tokens[2].substr(0, tokens[2].size() - 1));
        avail = stoi(tokens[3].substr(0, tokens[3].size() - 1));
        use_p = stoi(tokens[4].substr(0, tokens[4].size() - 1));
    }
};

vector<File> parse_files(const vector<string> &lines) {
    vector<File> files;
    files.reserve(lines.size());
    for (int i = 2; i < lines.size(); i++) {
        files.emplace_back(lines[i]);
    }
    return files;
}

int valid_pairs(const vector<File> &files) {
    int valid_pairs = 0;
    for (const auto &file_i: files) {
        if (file_i.used == 0) {
            continue;
        }
        for (const auto &file_j: files) {
            if (file_i.x == file_j.x && file_i.y == file_j.y) {
                continue;
            }
            if (file_i.used > file_j.avail) {
                continue;
            }
            valid_pairs++;
        }
    }
    return valid_pairs;
}


int main() {
    const auto data = read_lines("../../data/day22_data.txt");
    const auto files = parse_files(data);
    const int pairs = valid_pairs(files);

    cout << "Part 1: " << pairs << endl;
}
