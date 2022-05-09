using Xunit;

namespace Day18.Tests
{
    public class PairTests
    {
        [Fact]
        public void WhenParse_ThenCorrectPairs()
        {
            // Arrange
            var input = "[[[[[9,8],1],2],3],4]";

            // Act
            var pair = new Pair(input);

            // Assert
            Assert.Equal(4, pair.RightNumber);
            Assert.Null(pair.RightPair);
        }

        [Theory]
        [InlineData("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
        [InlineData("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
        [InlineData("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
        [InlineData("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")]
        [InlineData("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
        public void WhenExplode_ThenCorrectPairsAfter(
            string input, string expectedString)
        {
            // Arrange
            var pair = new Pair(input);
            var expected = new Pair(expectedString);

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Fact]
        public void WhenSplit_ThenCorrect()
        {
            // Arrange
            var pair = new Pair("[[[[0,7],4],[15,[0,13]]],[1,1]]");
            var expected = new Pair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Fact]
        public void WhenReduce_ThenCorrect()
        {
            // Arrange
            var pair = new Pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
            var expected = new Pair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Fact]
        public void WhenAdd_ThenCorrect()
        {
            // Arrange
            var firstPair = new Pair("[[[[4,3],4],4],[7,[[8,4],9]]]");
            var secondPair = new Pair("[1,1]");
            var pair = firstPair.Add(secondPair);
            var expected = new Pair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }
    }
}
