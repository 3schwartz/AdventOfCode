#include <sstream>
#include <string>
#include "../common/common.h""
#include "day21.h"


using std::istringstream;

/// swap position X with position Y
string handle_swap_position(string &password, const string &event) {
    istringstream iss(event);
    string word;
    size_t x, y;
    iss >> word >> word >> x >> word >> word >> y;
    const char x_c = password[x];
    const char y_c = password[y];
    password[x] = y_c;
    password[y] = x_c;
    return password;
}

/// swap letter X with letter Y
string handle_swap_letter(string &password, const string &event) {
    istringstream iss(event);
    string word;
    char x_c, y_c;
    iss >> word >> word >> x_c >> word >> word >> y_c;
    size_t x = password.find(x_c);
    size_t y = password.find(y_c);
    password[x] = y_c;
    password[y] = x_c;
    return password;
}

/// rotate based on position of letter X
string handle_rotate_based_on_position_of_letter(string &password, const string &event) {
    const string x_c = event.substr(35);
    size_t x = password.find(x_c);
    x += x >= 4 ? 2 : 1;

    std::string str;
    str.reserve(password.size());
    for (size_t i = 0; i < password.size(); ++i) {
        size_t idx = (i - x + password.size() + password.size()) % password.size();
        str += password[idx];
    }
    password = str;
    return password;
}

/// rotate left/right X steps
string handle_rotate_left_or_right(string &password, const string &event) {
    istringstream iss(event);
    string word, direction;
    size_t steps;
    iss >> word >> direction >> steps >> word;
    int direction_mult = direction == "right" ? -1 : 1;

    std::string str;
    str.reserve(password.size());
    for (size_t i = 0; i < password.size(); ++i) {
        size_t idx = (i + direction_mult * steps + password.size()) % password.size();
        str += password[idx];
    }
    password = str;
    return password;
}

/// reverse positions X through Y
string handle_reverse_positions(string &password, const string &event) {
    istringstream iss(event);
    string word;
    int start_idx, end_idx;
    iss >> word >> word >> start_idx >> word >> end_idx;
    std::reverse(password.begin() + start_idx, password.begin() + end_idx + 1);
    return password;
}

/// move position X to position Y
string handle_move_position(string &password, const string &event) {
    istringstream iss(event);
    string word;
    size_t x, y;
    iss >> word >> word >> x >> word >> word >> y;
    char c = password[x];
    password.erase(x, 1);
    password.insert(y, 1, c);
    return password;
}

string handle_event(string &password, const string &event) {
    if (event.starts_with("swap position")) {
        return handle_swap_position(password, event);
    }
    if (event.starts_with("swap letter ")) {
        return handle_swap_letter(password, event);
    }
    if (event.starts_with("rotate based on position of letter ")) {
        return handle_rotate_based_on_position_of_letter(password, event);
    }
    if (event.starts_with("rotate ")) {
        return handle_rotate_left_or_right(password, event);
    }
    if (event.starts_with("reverse positions ")) {
        return handle_reverse_positions(password, event);
    }
    if (event.starts_with("move position ")) {
        return handle_move_position(password, event);
    }

    throw std::runtime_error("Unknown event: " + event);
}
