#include <sstream>
#include <string>
#include "../common/common.h""
#include "day21.h"


using std::istringstream;
using std::move;

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

/// swap position X with position Y
string handle_swap_position_reverse(string &password, const string &event) {
    return handle_swap_position(password, event);
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

/// swap letter X with letter Y
string handle_swap_letter_reverse(string &password, const string &event) {
    return handle_swap_letter(password, event);
}

/// rotate based on position of letter X
string handle_rotate_based_on_position_of_letter(string &password, const string &event) {
    const string x_c = event.substr(35);
    size_t x = password.find(x_c);
    x += x >= 4 ? 2 : 1;

    std::string str;
    str.reserve(password.size());
    for (size_t i = 0; i < password.size(); ++i) {
        const size_t idx = (i - x + password.size() + password.size()) % password.size();
        str += password[idx];
    }
    password = str;
    return password;
}

/*
 * i:    0 1 2 3  4  5  6  7
 * add:  1 2 3 4  6  7  8  9
 * sum:  1 3 5 7 10 12 14 16
 * mod:  1 3 5 7  2  4  6  0
 *
 * What idx are i at after
 * P[i]: 7 0 4 1  5  2  6  3
 *
 * Shift from initial index to final index
 * P[i]-i: 7 -1 2 -2 1 -3 0 -4
 */
constexpr std::array rotate_based_on_position_of_letter_events = {
    7, -1, 2, -2, 1, -3, 0, -4
};
/*
 * i:    0 1 2 3  4
 * add:  1 2 3 4  6
 * sum:  1 3 5 7 10
 * mod:  1 3 0 2  0
 *
 * What idx are i at after
 * P[i]: 2 0 3 1 4
 *
 * Shift from initial index to final index
 * P[i]-i: 2 -1 1 -2 0
 */
constexpr std::array rotate_based_on_position_of_letter_events_test = {
    4, -1, 1, -2, 0
};


string handle_rotate_based_on_position_of_letter_reverse(string &password, const string &event, const bool test) {
    const string x_c = event.substr(35);
    const size_t x = password.find(x_c);
    const int rotate = test
                           ? rotate_based_on_position_of_letter_events_test[x]
                           : rotate_based_on_position_of_letter_events[x];

    std::string str(password.size(), '\0');
    for (size_t i = 0; i < password.size(); ++i) {
        const size_t idx = (i - rotate + password.size()) % password.size();
        str[i] = password[idx];
    }
    password = move(str);
    return password;
}

/// rotate left/right X steps
string handle_rotate_left_or_right(string &password, const string &event, const int rotate) {
    istringstream iss(event);
    string word, direction;
    size_t steps;
    iss >> word >> direction >> steps >> word;
    const int direction_mult = direction == "right" ? -1 * rotate : rotate;

    std::string str;
    str.reserve(password.size());
    for (size_t i = 0; i < password.size(); ++i) {
        size_t idx = (i + direction_mult * steps + password.size()) % password.size();
        str += password[idx];
    }
    password = move(str);
    return password;
}

string handle_rotate_left_or_right_reverse(string &password, const string &event) {
    return handle_rotate_left_or_right(password, event, -1);
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

string handle_reverse_positions_reverse(string &password, const string &event) {
    return handle_reverse_positions(password, event);
}

/// move position X to position Y
string handle_move_position(string &password, const string &event) {
    istringstream iss(event);
    string word;
    size_t x, y;
    iss >> word >> word >> x >> word >> word >> y;
    const char c = password[x];
    password.erase(x, 1);
    password.insert(y, 1, c);
    return password;
}

string handle_move_position_reverse(string &password, const string &event) {
    istringstream iss(event);
    string word;
    size_t x, y;
    iss >> word >> word >> x >> word >> word >> y;
    const char c = password[y];
    password.erase(y, 1);
    password.insert(x, 1, c);
    return password;
}

string handle_event(string &password, const string &event, const bool reverse) {
    if (event.starts_with("swap position")) {
        return reverse ? handle_swap_position_reverse(password, event) : handle_swap_position(password, event);
    }
    if (event.starts_with("swap letter ")) {
        return reverse ? handle_swap_letter_reverse(password, event) : handle_swap_letter(password, event);
    }
    if (event.starts_with("rotate based on position of letter ")) {
        return reverse
                   ? handle_rotate_based_on_position_of_letter_reverse(password, event)
                   : handle_rotate_based_on_position_of_letter(password, event);
    }
    if (event.starts_with("rotate ")) {
        return reverse
                   ? handle_rotate_left_or_right_reverse(password, event)
                   : handle_rotate_left_or_right(password, event);
    }
    if (event.starts_with("reverse positions ")) {
        return reverse ? handle_reverse_positions_reverse(password, event) : handle_reverse_positions(password, event);
    }
    if (event.starts_with("move position ")) {
        return reverse ? handle_move_position_reverse(password, event) : handle_move_position(password, event);
    }

    throw std::runtime_error("Unknown event: " + event);
}
