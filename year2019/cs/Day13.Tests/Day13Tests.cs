using Common;

namespace Day13.Tests;

public class Day13Tests
{
    [Fact]
    public void Part1()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day13_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();

        // Act
        var state = coder.PlayArcade(codes);

        // Assert
        Assert.Equal(344, state.BlockCount);
    }

    [Fact]
    public void Part2()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day13_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();
        codes[0] = 2;

        // Act
        var state = coder.PlayArcade(codes);

        // Assert
        Assert.Equal(344, state.BlockCount);
        Assert.Equal(17336, state.TotalScore);
    }
}