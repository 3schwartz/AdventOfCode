#include <format>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct Keypad
{
  pair<int, int> position;
  Keypad() : position(1, 1) {}

  void move(char direction)
  {
    switch (direction)
    {
    case 'U':
      position.second = position.second > 0 ? position.second - 1 : position.second;
      break;
    case 'D':
      position.second = position.second < 2 ? position.second + 1 : position.second;
      break;
    case 'L':
      position.first = position.first > 0 ? position.first - 1 : position.first;
      break;
    case 'R':
      position.first = position.first < 2 ? position.first + 1 : position.first;
      break;
    default:
      throw invalid_argument(format("Invalid instruction: {}", direction));
    }
  }

  string get_number()
  {
    if (position == pair{0, 0})
      return "1";
    else if (position == pair{1, 0})
      return "2";
    else if (position == pair{2, 0})
      return "3";
    else if (position == pair{0, 1})
      return "4";
    else if (position == pair{1, 1})
      return "5";
    else if (position == pair{2, 1})
      return "6";
    else if (position == pair{0, 2})
      return "7";
    else if (position == pair{1, 2})
      return "8";
    else if (position == pair{2, 2})
      return "9";

    throw invalid_argument(std::format("Invalid key position: {}, {}", position.first, position.second));
  }
};

int main()
{
  std::ifstream file("../../data/day2_data.txt");
  std::string line;
  std::vector<std::string> instructions;

  Keypad keypad = Keypad();

  while (std::getline(file, line))
  {
    for (char c : line)
    {
      keypad.move(c);
    }
    cout << keypad.get_number();
  }
}
