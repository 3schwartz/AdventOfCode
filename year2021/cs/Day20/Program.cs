using Day20;
using System.Diagnostics;

//Trace.Listeners.Add(new TextWriterTraceListener(Console.Out));

var lines = await File.ReadAllLinesAsync("../../../../../data/day20_data.txt");

var image = new Image(lines);

image.Enhance(2);
var pixelCount = image.GetPixelCount();

Console.WriteLine($"Part 1 : {pixelCount}");