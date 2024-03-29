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

        var diagnostocCode = coder.RunTest(codes, _ => 5);

        output.WriteLine($"Part 2 : {diagnostocCode}");
    }

    [Fact]
    public void WhenRunningPart1_ThenCorrect()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day5_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();

        // Act
        var diagnostocCode = coder.RunTest(codes, _ => 1);

        // Assert
        Assert.Equal(13087969, diagnostocCode);
    }
}