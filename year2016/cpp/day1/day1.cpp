
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
#include <fstream>

vector<string> split(const string &s) {
  vector<string> result;
  size_t start = 0, end;

  while ((end = s.find(", ", start)) != string::npos) {
    result.push_back(s.substr(start, end - start));
    start = end + 2;
  }

  result.push_back(s.substr(start));
  return result;
}

pair<int, int> visit_twice(const vector<string> &instructions) {
  set<pair<int, int>> visited = {{0, 0}};

  pair<int, int> direction = {0, 1};
  pair<int, int> position = {0, 0};
  for (const string &instruction : instructions) {
    switch (instruction[0]) {
    case 'R':
      direction = {direction.second, -direction.first};
      break;
    case 'L':
      direction = {-direction.second, direction.first};
      break;
    default:
      cerr << "Invalid instruction\n";
      exit(1);
    }
    int distance = stoi(instruction.substr(1));

    for (int i = 0; i < distance; i++) {
      position = {position.first + direction.first,
                  position.second + direction.second};
      if (!visited.insert(position).second) {
        return position;
      }
    }
  }
  cerr << "Not able to find a position visited twice" << endl;
  exit(1);
}

int main() {
  ifstream file("../../data/day1_data.txt");

  if (!file.is_open()) {
    cerr << "Failed to open file\n";
    exit(1);
  }

  string line;
  if (!getline(file, line)) {
    cerr << "Failed to read line\n";
    exit(1);
  }

  vector<string> instructions = split(line);

  pair<int, int> direction = {0, 1};
  pair<int, int> position = {0, 0};
  for (const string &instruction : instructions) {
    switch (instruction[0]) {
    case 'R':
      direction = {direction.second, -direction.first};
      break;
    case 'L':
      direction = {-direction.second, direction.first};
      break;
    default:
      cerr << "Invalid instruction\n";
      exit(1);
    }
    int distance = stoi(instruction.substr(1));
    position = {position.first + direction.first * distance,
                position.second + direction.second * distance};
  }
  cout << "Distance: " << abs(position.first) + abs(position.second) << endl;

  pair<int, int> position2 = visit_twice(instructions);
  cout << "Distance: " << abs(position2.first) + abs(position2.second) << endl;
}
