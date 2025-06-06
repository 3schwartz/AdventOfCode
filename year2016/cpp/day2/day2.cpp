#include <format>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

using std::format;
using std::invalid_argument;
using std::pair;
using std::string;
using std::vector;
using std::cout;
using std::ifstream;
using std::getline;
using std::endl;
using std::map;

class IKeyboard {
protected:
  ~IKeyboard() = default;

public:
  virtual string get_key() = 0;

  virtual void move(char direction) = 0;
};

class ComplexKeyboard final : public IKeyboard {
public:
  ComplexKeyboard() : position(0, 2) {
    keymap = {
      {pair{2, 0}, "1"},
      {pair{1, 1}, "2"},
      {pair{2, 1}, "3"},
      {pair{3, 1}, "4"},
      {pair{0, 2}, "5"},
      {pair{1, 2}, "6"},
      {pair{2, 2}, "7"},
      {pair{3, 2}, "8"},
      {pair{4, 2}, "9"},
      {pair{1, 3}, "A"},
      {pair{2, 3}, "B"},
      {pair{3, 3}, "C"},
      {pair{2, 4}, "D"}
    };
  }

private:
  pair<int, int> position;
  map<pair<int, int>, string> keymap;

public:
  string get_key() override {
    return keymap.at(position);
  }

  void move(char direction) override {
    switch (direction) {
      case 'U':
        if (position.first == 0 && position.second == 2 ||
            position.first == 1 && position.second == 1 ||
            position.first == 2 && position.second == 0 ||
            position.first == 3 && position.second == 1 ||
            position.first == 4 && position.second == 2) {
          break;
        }
        position.second--;
        break;
      case 'D':
        if (position.first == 0 && position.second == 2 ||
            position.first == 1 && position.second == 3 ||
            position.first == 2 && position.second == 4 ||
            position.first == 3 && position.second == 3 ||
            position.first == 4 && position.second == 2) {
          break;
        }
        position.second++;
        break;
      case 'R':
        if (position.first == 2 && position.second == 0 ||
            position.first == 3 && position.second == 1 ||
            position.first == 4 && position.second == 2 ||
            position.first == 3 && position.second == 3 ||
            position.first == 2 && position.second == 4) {
          break;
        }
        position.first++;
        break;
      case 'L':
        if (position.first == 2 && position.second == 0 ||
            position.first == 1 && position.second == 1 ||
            position.first == 0 && position.second == 2 ||
            position.first == 1 && position.second == 3 ||
            position.first == 2 && position.second == 4) {
          break;
        }
        position.first--;
        break;
      default:
        throw std::invalid_argument(format("Invalid instruction: {}", direction));
    }
  }
};

class SimpleKeypad final : public IKeyboard {
public:
  SimpleKeypad() : position(1, 1) {
  }

private:
  pair<int, int> position;

public:
  void move(char direction) override {
    switch (direction) {
      case 'U':
        position.second = std::max(0, position.second - 1);
        break;
      case 'D':
        position.second = std::min(2, position.second + 1);
        break;
      case 'L':
        position.first = std::max(0, position.first - 1);
        break;
      case 'R':
        position.first = std::min(2, position.first + 1);
        break;
      default:
        throw std::invalid_argument(format("Invalid instruction: {}", direction));
    }
  }

  string get_key() override {
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

void get_code(IKeyboard &keypad, const vector<string> &instructions) {
  for (const string &instruction: instructions) {
    for (char c: instruction) {
      keypad.move(c);
    }
    cout << keypad.get_key();
  }
  cout << endl;
}

int main() {
  ifstream file("../../data/day2_data.txt");
  string line;
  vector<std::string> instructions;
  while (getline(file, line)) {
    instructions.push_back(line);
  }

  SimpleKeypad keypad = SimpleKeypad();

  get_code(keypad, instructions);

  ComplexKeyboard complex_keyboard = ComplexKeyboard();

  get_code(complex_keyboard, instructions);
}
