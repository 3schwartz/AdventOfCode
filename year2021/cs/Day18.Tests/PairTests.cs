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
    }
}
