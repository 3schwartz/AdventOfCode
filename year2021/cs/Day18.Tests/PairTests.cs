using Common;
using System.Linq;
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
        [InlineData("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
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
        public void WhenAdd_ThenConstructCorrect()
        {
            // Arrange
            var firstPair = new Pair("[[[[4,3],4],4],[7,[[8,4],9]]]");
            var secondPair = new Pair("[1,1]");
            var expected = new Pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

            // Act
            var pair = firstPair.Add(secondPair);

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")]
        [InlineData("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")]
        public void WhenAdd_ThenReduceCorrect(
            string first, string second, string expectedString)
        {
            // Arrange
            var firstPair = new Pair(first);
            var secondPair = new Pair(second);
            var expected = new Pair(expectedString);
            var pair = firstPair.Add(secondPair);

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[9,1],[1,9]]", 129)]
        [InlineData("[[1,2],[[3,4],5]]", 143)]
        [InlineData("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
        [InlineData("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
        [InlineData("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
        [InlineData("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
        [InlineData("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
        public void WhenCalculateMagnitude_ThenCorrect(string input, int magnitude)
        {
            // Arrange
            var pair = new Pair(input);

            // Act
            var actual = pair.CalculateMagnitude();

            // Assert
            Assert.Equal(magnitude, actual);
        }

        [Theory]
        [InlineData(2, "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")]
        //[InlineData(10, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")]
        public void WhenCalculateFinalSum_ThenCorrect(
            int lines, string expected)
        {
            // Arrange
            var numbers = DataLoader.GetData("../../../../../data/day18_data_test.txt")
                .Select(d => new Pair(d)).ToList();
            var expectedMagnitude = new Pair(expected).CalculateMagnitude();
            
            // Act
            var number = numbers[0];
            for (var i = 1; i < lines; i++)
            {
                number = number.Add(numbers[i]);
                number.Reduce();
            }

            var magnitude = number.CalculateMagnitude();

            // Assert
            Assert.Equal(expectedMagnitude, magnitude);
        }
    }
}
