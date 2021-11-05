#include "day3_func.h"

using namespace std;

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

int getMinimumDistance(vector<string> wireOneInput, vector<string> wireTwoInput) {
	map<tuple<int, int>, int> wireOne = getCoordinates(wireOneInput);
	map<tuple<int, int>, int> wireSecond = getCoordinates(wireTwoInput);

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

	return minimalDistance;
}

int getMinimumSteps(std::vector<std::string> wireOneInput, std::vector<std::string> wireTwoInput) {
	map<tuple<int, int>, int> wireOne = getCoordinates(wireOneInput);
	map<tuple<int, int>, int> wireSecond = getCoordinates(wireTwoInput);

	set<tuple<int, int>> placesWireOne = getSetFromKeys(wireOne);
	set<tuple<int, int>> placesWireSecond = getSetFromKeys(wireSecond);

	set<tuple<int,int>> intersect;
	set_intersection(placesWireOne.begin(), placesWireOne.end(),
					placesWireSecond.begin(), placesWireSecond.end(),
					std::inserter(intersect, intersect.begin()));
	
	vector<int> steps;

	for(auto elem : intersect) {
		if (elem != make_tuple(0,0)) {
			steps.push_back(wireOne[elem] + wireSecond[elem]);
		}
	}

	int minimalSteps = *min_element(steps.begin(), steps.end());

	return minimalSteps;
}