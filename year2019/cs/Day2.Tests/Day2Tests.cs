using Common;
using Xunit;

namespace Day2.Tests;

public class Day2Tests
{
    [Fact]
    public void WhenRunningInitialIntCodeProgram_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllText("../../../../../data/day2_data.txt").Split(",");
        var coder = new IntCoder();
        var codes = lines.Select(c => int.Parse(c)).ToList();

        // Act
        coder.RunInstructions(codes, 12, 2);

        // Assert
        Assert.Equal(5305097, codes[0]);
    }

    [Fact]
    public void WhenFindingOutput_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllText("../../../../../data/day2_data.txt").Split(",");
        var coder = new IntCoder();
        var codes = lines.Select(c => int.Parse(c)).ToList();

        // Act
        var optimal = coder.FindOutput(codes);

        // Assert
        Assert.Equal(4925, optimal);
    }
}