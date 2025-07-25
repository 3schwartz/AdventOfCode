#ifndef DAY21_H
#define DAY21_H
#include <string>

using std::string;

string handle_swap_position(string &password, const string &event);

string handle_swap_position_reverse(string &password, const string &event);

string handle_swap_letter(string &password, const string &event);

string handle_swap_letter_reverse(string &password, const string &event);

string handle_rotate_based_on_position_of_letter(string &password, const string &event);

string handle_rotate_based_on_position_of_letter_reverse(string &password, const string &event, bool test = false);

string handle_rotate_left_or_right(string &password, const string &event, int rotate = 1);

string handle_rotate_left_or_right_reverse(string &password, const string &event);

string handle_reverse_positions(string &password, const string &event);

string handle_reverse_positions_reverse(string &password, const string &event);

string handle_move_position(string &password, const string &event);

string handle_move_position_reverse(string &password, const string &event);

string handle_event(string &password, const string &event, bool reverse = false);

#endif //DAY21_H
