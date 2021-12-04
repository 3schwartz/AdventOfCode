#include "../include/day2_func.h"
#include <fstream>
#include <iostream>

using namespace std;

int main() {
    ifstream ifs("../../../data/day2_data.txt");
    string line;
    vector<Move> moves;
    while(getline(ifs, line)) {
        moves.push_back(Move(line));
    }

    RouteCalculator routeCalculator;
    int positiion = routeCalculator.GetPosition(moves);

    cout << "Part 1: " << positiion << endl;

    AimCalculator aimCalculator;
    int aimPosition = aimCalculator.GetPosition(moves);

    cout << "Part 2: " << aimPosition << endl;
}