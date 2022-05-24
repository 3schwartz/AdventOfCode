using System.Collections.Generic;
using System.IO;
using System.Threading.Tasks;
using Xunit;

namespace Day22.Tests;

public class Day22Tests
{
    [Fact]
    public async Task GivenLimit_WhenGivenSteps_ThenSwitchLights()
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync("../../../../../data/day22_data_test.txt");
        
        // Act
        var lightsOn = new LightSwitcher().GetOnLights(lines);

        // Assert
        Assert.Equal(590784, lightsOn);
    }

    [Fact]
    public void WhenGivenSteps_ThenSwitchLights()
    {
        // Arrange
        var steps = new[]
        {
            "on x=10..12,y=10..12,z=10..12",
            "on x=11..13,y=11..13,z=11..13",
            "off x=9..11,y=9..11,z=9..11",
            "on x=10..10,y=10..10,z=10..10"
        };

        // Act
        var lightsOn = new LightSwitcher().GetOnLights(steps);

        // Assert
        Assert.Equal(39, lightsOn);

    }

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