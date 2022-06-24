using System.IO;
using Xunit;
using System.Linq;
using Xunit.Abstractions;
using Common;

namespace Day5.Tests;

public partial class Day5Tests
{
    private readonly ITestOutputHelper output;

    public Day5Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Program()
    {
        var codes = IntCoder.InputToCodes(File.ReadAllText("../../../../../data/day5_data.txt"));

        var coder = new IntCoder();

        int diagnostocCode = coder.RunTest(codes, _ => 5);

        output.WriteLine($"Part 1 : {diagnostocCode}");
    }

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

    [Fact]
    public void WhenRunningTest_ThenCorrect()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day5_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();

        // Act
        int diagnostocCode = coder.RunTest(codes, _ => 1);

        // Assert
        Assert.Equal(13087969, diagnostocCode);
    }
}