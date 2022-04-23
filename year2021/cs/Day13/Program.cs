using Day1;

// For test data
//var v = File.ReadAllText("../../../../../data/day13_data_test.txt");
var v = File.ReadAllText("../../../../../data/day13_data.txt");
var vs = v.Split("\r\n\r\n");


var foldsAsString = vs[1].Split("\r\n");
var folds = new Fold[foldsAsString.Length];

for (var i = 0; i < foldsAsString.Length; i++)
{
    folds[i] = FoldFactory.CreateFold(foldsAsString[i]);
}

var grid = new Grid(655 * 2 + 1, 447 * 2 + 1);
// For test data
//var grid = new Grid(11,15);
var coords = CoordinateFactory.CreateCoordinates(vs[0]);
grid.PopulateGrid(coords);

for (var i = 0; i < folds.Length; i++)
{
    grid.Fold(folds[i]);

    if (i == 0)
    {
        Console.WriteLine($"Part 1: {grid.GetNumberOfDots()}");
    }
}

Console.WriteLine("Part 2:");
grid.Print();
