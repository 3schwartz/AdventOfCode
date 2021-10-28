#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <map>
#include <sstream>
#include <string>
#include <tuple>

using namespace std;

vector<string> createVectorFromLine(string line) {
	replace(line.begin(), line.end(), ',', ' ');

	stringstream ss(line);

	vector<string> input;
	string v;
	while (ss >> v) {
		input.push_back(v);
	}

	return input;
}

map<int, vector<string>> readInput() {

	ifstream ifs("../../data/day3_data.txt");
	string line;
	vector<string> lines;
	while (ifs >> line) {
		lines.push_back(line);
	}

	map<int, vector<string>> wires;

	for (int i = 0; i < lines.size(); i++) {
		string line = lines[i];

		vector<string> input = createVectorFromLine(line);

		wires[i] = input;
	}

	//for (auto const& x : wires)
	//{
	//	cout << x.first << ':' << x.second.size() << std::endl;
	//}

	return wires;
}

void addSteps(map<char, int> coordinates, char coord, int multiplier, int value, map<tuple<int, int>, int> places) {
	int position_before = coordinates[coord];
	int position_after = position_before + multiplier * value;

	int steps = coordinates['s'] + 1;

	for (int i = position_before + multiplier; i < position_after + multiplier; i += multiplier) {
		tuple<int, int> place;
		if (coord == 'x') {
			place = make_tuple(i, coordinates['y']);
		}
		else {
			place = make_tuple(coordinates['x'], i);
		}
		places[place] = steps;
		steps++;
	}

	if (coord == 'x') {
		coordinates['x'] += multiplier * value;
	}
	else {
		coordinates['y'] += multiplier * value;
	}

	coordinates['s'] += value;
}

map<tuple<int, int>, int> getCoordinates(vector<string> wire) {
	map<char, int> coordinates = {
		{'x', 0},
		{'y', 0},
		{'s', 0}
	};
	map<tuple<int, int>, int> places;

	for (auto &direction : wire) {
		string key = direction.substr(0, 1);
		int value = stoi(direction.substr(1, direction.size()));

		if (key == "U") {
			addSteps(coordinates, 'x', 1, value, places);
		}
		if (key == "D") {
			addSteps(coordinates, 'x', -1, value, places);
		}
		if (key == "R") {
			addSteps(coordinates, 'y', 1, value, places);
		}
		if (key == "L") {
			addSteps(coordinates, 'y', -1, value, places);
		}

		cout << key << " : " << value << endl;
	}

	return places;
}

int main() {
	map<int, vector<string>> wires = readInput();

	map<tuple<int, int>, int> wireOne = getCoordinates(wires[0]);
	map<tuple<int, int>, int> wireSecond = getCoordinates(wires[1]);

	/*for (auto &wire : wires) {
		getCoordinates(wire.second);
	}*/

	return 0;

}

