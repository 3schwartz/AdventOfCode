using Day20;
using System.Diagnostics;

Trace.Listeners.Add(new TextWriterTraceListener(Console.Out));

var lines = await File.ReadAllLinesAsync("../../../../../data/day20_data_test.txt");

var algorithm = Image.CreateImageEnchancementAlgorithm(lines[0]);