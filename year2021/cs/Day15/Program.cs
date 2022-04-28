using Common;
using Day15;

var data = DataLoader.GetData("../../../../../data/day15_data.txt");

var pathFinder = new PriorityQueueFinder();

var nodes = pathFinder.CreateNodes(data, 1);
var minimum = pathFinder.FindShortest(nodes);

Console.WriteLine($"Part 1: {minimum}");

var nodesBig = pathFinder.CreateNodes(data, 5);
var minimumBig = pathFinder.FindShortest(nodesBig);

Console.WriteLine($"Part 2: {minimumBig}");