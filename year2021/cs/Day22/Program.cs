using Day22;

var lines = await File.ReadAllLinesAsync("../../../../../data/day22_data.txt");

var lightSwitcher = new LightIntervalSwitcher(lines);

var lightsOnWithLimit = lightSwitcher.GetOnLights(true);

Console.WriteLine($"Part 1: {lightsOnWithLimit}");

var lightsOn = lightSwitcher.GetOnLights(false);

Console.WriteLine($"Part 2: {lightsOn}");



