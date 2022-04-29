using Day17;

var input = File.ReadAllText("../../../../../data/day17_data.txt");

var launcher = new Launcher(input);
var yMax = launcher.GetMaxHorizontalVelocity();

Console.WriteLine($"Part 1: {yMax.yMaxHeight}");

var velocitiesCount = launcher.GetVelocitiesCount();

Console.WriteLine($"Part 2: {velocitiesCount}");