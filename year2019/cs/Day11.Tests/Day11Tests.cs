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
    public void Part1()
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
        Assert.Equal(2415, outputs.Count);
    }


    [Fact]
    public void Part2()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day11_data.txt")
            .Split(",")
            .Select(c => long.Parse(c))
            .ToList();

        var coder = new IntCoder();
        var visited = new DefaultDict<(int X, int Y), int>
        {
            { (0, 0), 1 }
        };

        // Act
        coder.PaintHullWithInput(codes, visited);

        // Assert
        coder.OutputHullPaint(visited, (output) => this.output.WriteLine(output));
    }
}