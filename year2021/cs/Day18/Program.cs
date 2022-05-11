using Common;
using Day18;

var strings = DataLoader.GetData("../../../../../data/day18_data.txt");
var numbers = strings.Select(d => new Pair(d)).ToList();

var homework = Pair.CalculateHomework(numbers);

Console.WriteLine($"Part 1: {homework}");

var maxMagnitude = Pair.CalculateMaxMagnitude(strings);

Console.WriteLine($"Part 2: {maxMagnitude}");