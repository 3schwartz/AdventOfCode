using Day23;

var rooms = new List<char>()
    {'D', 'B', 'A', 'C', 'D', 'B', 'C', 'A'};
var roomSize = 2;
var sorter = new AmphipodStringSorter(roomSize);

var leastEnergy = sorter.CalculateLeastEnergy(rooms);

Console.WriteLine($"Part 1: {leastEnergy}");

var roomsBig = new List<char>()
    {'D', 'D','D','B', 'A','C','B', 'C', 'D','B','A', 'B', 'C','A','C', 'A'};
var roomSizeBig = 4;
var sorterBig = new AmphipodStringSorter(roomSizeBig);

var leastEnergyBig = sorterBig.CalculateLeastEnergy(roomsBig);

Console.WriteLine($"Part 2: {leastEnergyBig}");