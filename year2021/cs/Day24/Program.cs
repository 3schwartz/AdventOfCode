using Day24;

//var alu = new ArithmeticLogicUnit();

//var lines = await File.ReadAllLinesAsync("../../../../../data/day24_data.txt");

//var highstMonad = alu.FindHighestMonad(lines);

//Console.WriteLine($"Part 1: {highstMonad.Highest}");


// Thanks: https://github.com/dphilipson/advent-of-code-2021/blob/master/src/days/day24.rs

var lines = await File.ReadAllLinesAsync("../../../../../data/day24_data.txt");

for (int i = 0; i < lines.Length; i++)
{
    switch (i % 18)
    {
        case 0:
            Console.WriteLine($"#### Repeat: {i / 18} #####");
            break;
        case 5:
            Console.WriteLine($"x add: {lines[i]}");
            break;
        case 15:
            Console.WriteLine($"y add: {lines[i]}");
            break;
        default:
            break;
    }

}