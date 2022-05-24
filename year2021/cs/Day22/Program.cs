using Day22;

var lines = await File.ReadAllLinesAsync("../../../../../data/day22_data.txt");

var lightsOn = new LightSwitcher().GetOnLights(lines);

Console.WriteLine($"Part 1: {lightsOn}");