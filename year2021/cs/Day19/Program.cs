using Day19.Tests;

var scannersText = await File.ReadAllTextAsync("../../../../../data/day19_data.txt");
var scanners = Scanner.CreateScanners(scannersText);
var beacons = Scanner.FindBeacons(scanners);

Console.WriteLine($"Part 1: {beacons.Count}");

