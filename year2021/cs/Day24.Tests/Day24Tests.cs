using Xunit;

namespace Day24.Tests;

public class Day24Tests
{
    [Theory]
    [InlineData(3, 1)]
    [InlineData(2, 0)]
    public void WhenGivenInstructions_ThenCorrectUnits(int x, int expected)
    {
        // Arrange
        var instructions = new string[] { "inp z", "inp x", "mul z 3", "eql z x" };
        var alu = new ArithmeticLogicUnit();

        // Act       
        var run = alu.RunMonad(alu.InitInstructions(instructions),
            $"1{x}");

        // Assert

        Assert.True(run.Match(r => expected == r["z"], _ => false));
    }
}