using Common;
using Day15;

var data = DataLoader.GetData("../../../../../data/day15_data.txt");

var pathFinder = new DijkstraFinder();

var nodes = pathFinder.CreateNodes(data);
var minimum = pathFinder.FindShortest(nodes);

Console.WriteLine($"Part 1: {minimum}");