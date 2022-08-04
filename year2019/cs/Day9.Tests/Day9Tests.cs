using Common;

namespace Day9.Tests;

public class Day9Tests
{
    [Theory]
    [InlineData(1, 3460311188)]
    [InlineData(2, 42202)]
    public void WhenTestSolution_ThenCorrect(int input, long expected)
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day9_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();

        // Act
        var actual = coder.RunTest(codes, _ => input);

        // Assert
        Assert.Equal(expected, actual);
    }
}