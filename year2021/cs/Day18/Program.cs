using Common;
using Day18;

var numbers = DataLoader.GetData("../../../../../data/day18_data.txt")
    .Select(d => new Pair(d)).ToList();

var homework = Pair.CalculateHomework(numbers);

Console.WriteLine($"Part 1: {homework}");