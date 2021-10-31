#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <tuple>
#include <cmath>

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

	return wires;
}

void addSteps(map<char, int> &coordinates, char coord, int multiplier, int value, map<tuple<int, int>, int> &places, int steps) {
	int position_before = coordinates[coord];
	int position_after = position_before + multiplier * value;

	auto loopCondition = [](int i, int multiplier, int position_after) {
		return (multiplier > 0 ? i < position_after : i > position_after + multiplier);
	};

	for (int i = position_before + multiplier; loopCondition(i, multiplier, position_after); i += multiplier) {
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

		int steps = coordinates['s'] + 1;

		if (key == "U") {
			addSteps(coordinates, 'x', 1, value, places, steps);
		}
		else if (key == "D") {
			addSteps(coordinates, 'x', -1, value, places, steps);
		}
		else if (key == "R") {
			addSteps(coordinates, 'y', 1, value, places, steps);
		}
		else if (key == "L") {
			addSteps(coordinates, 'y', -1, value, places, steps);
		} else {
			cout << "Error: " << key << endl;
		}
	}

	return places;
}

template <class T, class U>
set<T> getSetFromKeys(map<T,U> mapWithKeys){
	
	set<T> outputSet;
	
	for (pair<const T, U> keyValuePair : mapWithKeys) {
		outputSet.insert(keyValuePair.first);
	}

	return outputSet;	
}

int main() {
	map<int, vector<string>> wires = readInput();

	map<tuple<int, int>, int> wireOne = getCoordinates(wires[0]);
	map<tuple<int, int>, int> wireSecond = getCoordinates(wires[1]);

	set<tuple<int, int>> placesWireOne = getSetFromKeys(wireOne);
	set<tuple<int, int>> placesWireSecond = getSetFromKeys(wireSecond);

	set<tuple<int,int>> intersect;
	set_intersection(placesWireOne.begin(), placesWireOne.end(),
					placesWireSecond.begin(), placesWireSecond.end(),
					std::inserter(intersect, intersect.begin()));
	
	vector<int> distances;
	for (auto elem : intersect) {
		int xCoordinate = get<0>(elem);
		int yCoordinate = get<1>(elem);

		if (xCoordinate != 0 || yCoordinate != 0) {
			distances.push_back(abs(xCoordinate) + abs(yCoordinate));
		}
	}

	int minimalDistance = *min_element(distances.begin(), distances.end());

	cout << "Part 1: " << minimalDistance << endl;

	vector<int> steps;
	for(auto elem : intersect) {
		if (elem != make_tuple(0,0)) {
			// cout << wireOne[elem] + wireSecond[elem] << endl;
			steps.push_back(wireOne[elem] + wireSecond[elem]);
		}
	}

	int minimalSteps = *min_element(steps.begin(), steps.end());

	cout << "Part 2: " << minimalSteps << endl;

	/*for (auto &wire : wires) {
		getCoordinates(wire.second);
	}*/

	return 0;

}