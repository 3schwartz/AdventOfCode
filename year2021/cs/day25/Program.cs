using day25;

var lines = await File.ReadAllLinesAsync("../../../../../data/day25_data.txt");
var cucumber = new SeaCucumberZone(lines);

cucumber.MoveToSteadyState();

Console.WriteLine($"Part 1 : {cucumber.Steps}");