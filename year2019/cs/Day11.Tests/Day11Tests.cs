using Common;
using Xunit.Abstractions;

namespace Day11.Tests;

public class Day11Tests
{
    private readonly ITestOutputHelper output;

    public Day11Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void WhenTestSolution_ThenCorrect()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day11_data.txt")
            .Split(",")
            .Select(c => long.Parse(c))
            .ToList();
        
        var coder = new IntCoder();

        // Act
        var outputs = coder.PaintHull(codes);

        // Assert
        output.WriteLine($"Part 1: {outputs.Count}");
    }
}