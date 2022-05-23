using System;
using Xunit;

namespace Day22.Tests;

public class Day22Tests
{
    [Theory]
    [InlineData("On x=10..12,y=10..12,z=10..12",Turn.On,10,12,10,12,10,12)]
    [InlineData("On x=11..13,y=11..13,z=11..13", Turn.On, 11,13,11,13,11,13)]
    [InlineData("Off x=9..11,y=9..11,z=9..11", Turn.Off, 9,11,9,11,9,11)]
    [InlineData("On x=10..10,y=10..10,z=10..10", Turn.On, 10,10,10,10,10,10)]
    public void WhenGivenLine_ThenCorrectFormat(string line, Turn turn,
        int xFrom, int xTo,
        int yFrom, int yTo,
        int zFrom, int zTo)
    {
        // Act
        var instruction = InstructionCreator.Create(line);

        // Assert
        Assert.Equal(turn, instruction.Turn);
        Assert.Equal(xFrom, instruction.XFrom);
        Assert.Equal(xTo, instruction.XTo);
        Assert.Equal(yFrom, instruction.YFrom);
        Assert.Equal(yTo, instruction.YTo);
        Assert.Equal(zFrom, instruction.ZFrom);
        Assert.Equal(zTo, instruction.ZTo);

    }
}

public enum Turn
{
    On,
    Off
}

internal record struct Instruction(Turn Turn, int XFrom, int XTo, int YFrom, int YTo, int ZFrom, int ZTo);

internal static class InstructionCreator
{
    internal static Instruction Create(string line)
    {
        var initial = line.Split(" ");
        Enum.TryParse(initial[0], true, out Turn turn);

        var coordinates = initial[1].Split(",");
        var x = GetFromTo(coordinates[0]);
        var y = GetFromTo(coordinates[1]);
        var z = GetFromTo(coordinates[2]);
        return new Instruction(
            turn,
            x.Item1, x.Item2,
            y.Item1, y.Item2,
            z.Item1, z.Item2);
    }

    private static (int, int) GetFromTo(string coordinate)
    {
        var initial = coordinate.Split("=");
        var coordinates = initial[1].Split("..");
        return (int.Parse(coordinates[0]), int.Parse(coordinates[1]));
    }
}