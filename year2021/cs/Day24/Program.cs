using Day24;

var alu = new ArithmeticLogicUnit();

var lines = await File.ReadAllLinesAsync("../../../../../data/day24_data.txt");

var highstMonad = alu.FindHighestMonad(lines);

Console.WriteLine($"Part 1: {highstMonad.Highest}");
