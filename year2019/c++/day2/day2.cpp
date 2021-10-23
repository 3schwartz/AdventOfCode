#include <iostream>
#include <vector>
#include <fstream>
#include <string>
#include <algorithm>
#include <sstream>

using namespace std;

int intCodeProgram(vector<int> input);
vector<int> readInput();

void partOne(vector<int> input) {
	input[1] = 12;
	input[2] = 2;

	int instruction = intCodeProgram(input);

	cout << "Part 1: " << instruction << endl;

	return;
}

void partTwo(vector<int> input) {
	bool found = false;
	int i;
	int j;

	for (i = 0; i < 100; i++) {
		for (j = 0; j < 100; j++) {
			vector<int> newInput = input;

			newInput[1] = i;
			newInput[2] = j;

			int instruction = intCodeProgram(newInput);

			if (instruction == 19690720) {
				found = true;
				break;
			}
		}

		if (found) {
			break;
		}
	}

	cout << "Part 2: " << 100 * i + j << endl;
}

int main() {
	vector<int> input = readInput();

	partOne(input);
	partTwo(input);

	return 0;
}

vector<int> readInput() {

	vector<string> lines;

	ifstream ifs("../../data/day2_data.txt");
	string line; ifs >> line;
	ifs.close();

	replace(line.begin(), line.end(), ',', ' ');

	cout << line;

	istringstream ss(line);

	vector<int> input;
	int v;
	while (ss >> v) {
		input.push_back(v);
	}

	return input;
}

int intCodeProgram(vector<int> input) {
	for (unsigned int i = 0; i < input.size(); i += 4) {
		int intCode = input[i];

		if (intCode == 1) {
			input[input[i + 3]] = input[input[i + 1]] + input[input[i + 2]];
		}
		else if (intCode == 2) {
			input[input[i + 3]] = input[input[i + 1]] * input[input[i + 2]];
		}
		else if (intCode == 99) {
			break;
		}
	}

	return input[0];
}