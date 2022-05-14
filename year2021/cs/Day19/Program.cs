using Day19;

var scannersText = await File.ReadAllTextAsync("../../../../../data/day19_data.txt");
var scanners = Scanner.CreateScanners(scannersText);
var beacons = Scanner.FindBeacons(scanners);

Console.WriteLine($"Part 1: {beacons.Beacons.Count}");

int distance = Scanner.GetLargestManhattenDistance(beacons.ScannerPositions);

Console.WriteLine($"Part 2: {distance}");

