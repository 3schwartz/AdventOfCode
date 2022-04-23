using Day14;

var polymerInserter = new PolymerInserter();

var data = polymerInserter.GetData("../../../../../data/day14_data.txt");

var polymerTemplate = polymerInserter.DoInsertion(data, 10);
int mostCommonMinusLeastCommon = polymerInserter.GetMostCommonMinusLeastCommon(polymerTemplate);

Console.WriteLine($"Part 1: {mostCommonMinusLeastCommon}");

var polymerTemplateBig = polymerInserter.DoInsertion(data, 40);
int mostCommonMinusLeastCommonBig = polymerInserter.GetMostCommonMinusLeastCommon(polymerTemplateBig);

Console.WriteLine($"Part 2: {mostCommonMinusLeastCommonBig}");