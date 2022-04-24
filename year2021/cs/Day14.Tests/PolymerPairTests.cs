using System.Collections.Generic;
using Xunit;
using static Day14.PolymerPair;

namespace Day14.Tests
{
    public class PolymerPairTests
    {
        private readonly PolymerPair polymer;

        public PolymerPairTests()
        {
            polymer = new PolymerPair();
        }

        [Theory]
        [InlineData(10, 1588)]
        [InlineData(40, 2188189693529)]
        public void WhenInsertRulesLoop_ThenCorrect(int insertions, long expected)
        {
            // Arrange
            var data = DataLoader.GetData("../../../../../data/day14_data_test.txt");

            // Act
            var polymerPairs = polymer.UpdatePairs(data, insertions);
            var mostCommonMinusLeastCommon = polymer.MostMinusLeastFromPairs(polymerPairs);

            // Act
            Assert.Equal(expected, mostCommonMinusLeastCommon);
        }

        [Theory]
        [InlineData("NCNBCHB", 1)]
        [InlineData("NBCCNBBBCBHCB", 2)]
        [InlineData("NBBBCNCCNBBNBNBBCHBHHBCHB", 3)]
        [InlineData("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", 4)]
        public void WhenInsert_ThenUpdateCorrect(string expected, int iterations)
        {
            // Arrange
            var data = DataLoader.GetData("../../../../../data/day14_data_test.txt");
            var expectedMaxMin = new PolymerInserter().GetMostCommonMinusLeastCommon(expected);

            // Act
            var polymerPairs = polymer.UpdatePairs(data, iterations);
            var mostCommonMinusLeastCommon = polymer.MostMinusLeastFromPairs(polymerPairs);

            // Act
            Assert.Equal(expectedMaxMin, mostCommonMinusLeastCommon);
        }

        [Fact]
        public void WhenGivenLineRules_ThenCreateDictRules()
        {
            // Arrange
            var lineRules = new string[2] { "HB -> C", "HC -> B" };

            // Act
            IList<DictRule> rules = polymer.CreateDictRules(lineRules);

            // Assert
            Assert.Equal(2, rules.Count);
            Assert.Equal("HB", rules[0].Rule);
            Assert.Equal("HC", rules[0].FirstInsert);
            Assert.Equal("CB", rules[0].SecondInsert);
        }

        [Fact]
        public void WhenCreateInitial_ThenReturnPairs()
        {
            // Arrange
            var template = "NHHCF";
            var rules = new List<DictRule> { new DictRule("HH", "HF", "FH", 'F'), new DictRule("NH", "NQ", "QH", 'Q') };

            // Act
            IDictionary<string, long> pairs = polymer.CreatePairs(template, rules);

            // Assert
            var expected = new Dictionary<string, long>
            {
                {"NH", 1 },
                {"HH", 1 },
                {"HC", 1 },
                {"CF", 1 },
                {"HF", 0 },
                {"FH", 0 },
                {"NQ", 0 },
                {"QH", 0 }
            };
            Assert.Equal(expected, pairs);
        }


        [Fact]
        public void WhenUpdatePairs_ThenReturnUpdatedFromRules()
        {
            // Arrange
            var pairs = new Dictionary<string, long>
            {
                {"NH", 1 },
                {"HH", 1 },
                {"HC", 1 },
                {"CF", 1 },
                {"HF", 0 },
                {"FH", 0 },
                {"NQ", 0 },
                {"QH", 0 }
            };
            var rules = new List<PairUpdate> {
                new PairUpdate(new DictRule("HH", "HF", "FH", 'F'), 0),
                new PairUpdate(new DictRule("NH", "NQ", "QH", 'Q'), 0)
            };
            var charCount = new Dictionary<char, long>()
            {
                {'N', 0}, {'H', 0}, {'Q', 0}, {'F', 0}, {'C', 0},
            };

            // Act
            polymer.UpdatePairsFromRules(pairs, rules, charCount);

            // Assert
            var expected = new Dictionary<string, long>
            {
                {"NH", 0 },
                {"HH", 0 },
                {"HC", 1 },
                {"CF", 1 },
                {"HF", 1 },
                {"FH", 1 },
                {"NQ", 1 },
                {"QH", 1 }
            };
            Assert.Equal(expected, pairs);
            Assert.Equal(1, charCount['F']);
            Assert.Equal(1, charCount['Q']);
        }
    }
}
