using Day14;

var polymerInserter = new PolymerInserter();

var data = DataLoader.GetData("../../../../../data/day14_data.txt");

var polymerTemplate = polymerInserter.DoInsertion(data, 10);
int mostCommonMinusLeastCommon = polymerInserter.GetMostCommonMinusLeastCommon(polymerTemplate);

Console.WriteLine($"Part 1: {mostCommonMinusLeastCommon}");

var polymerPair = new PolymerPair();
var polymerTemplateBig = polymerPair.UpdatePairs(data, 40);
var mostCommonMinusLeastCommonBig = polymerPair.MostMinusLeastFromPairs(polymerTemplateBig);

Console.WriteLine($"Part 2: {mostCommonMinusLeastCommonBig}");