using Day20;
using System.Diagnostics;

//Trace.Listeners.Add(new TextWriterTraceListener(Console.Out));

var lines = await File.ReadAllLinesAsync("../../../../../data/day20_data.txt");

var image = new ImageSet(lines);

image.Enhance(2);
var pixelCount = image.GetPixelCount();

Console.WriteLine($"Part 1 : {pixelCount}");

//image.Enhance(48);
//pixelCount = image.GetPixelCount();

//Console.WriteLine($"Part 2 : {pixelCount}");