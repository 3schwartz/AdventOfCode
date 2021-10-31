#include <iostream>
#include <fstream>
#include <algorithm>
#include <sstream>
#include <map>
#include <vector>
#include "day3_func.h"

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

int main() {
	map<int, vector<string>> wires = readInput();

	cout << "Part 1: " << getMinimumDistance(wires[0], wires[1]) << endl;

	cout << "Part 2: " << getMinimumSteps(wires[0], wires[1])  << endl;

	return 0;

}