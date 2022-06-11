using Xunit;
using System.Collections.Generic;

namespace Day24.Tests;

public class Day24
{
    [Theory]
    [InlineData(3, 1)]
    [InlineData(2, 0)]
    public void WhenGivenInstructions_ThenCorrectUnits(int x, int expected)
    {
        // Arrange
        string[] instructions = new string[] { "inp z", "inp x", "mul z 3", "eql z x" };
        var alu = new ArithmeticLogicUnit();

        // Act
        IReadOnlyDictionary<string, int> run = alu.RunMonad(instructions, $"1{x}");

        // Assert
        Assert.Equal(expected, run["z"]);
    }
}