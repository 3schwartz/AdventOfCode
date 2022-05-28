using System.IO;
using System.Threading.Tasks;
using Xunit;

namespace Day22.Tests;

public class Day22Tests
{
    [Theory]
    [InlineData("../../../../../data/day22_data_test1.txt", true, 590784)]
    [InlineData("../../../../../data/day22_data_test2.txt", false, 2758514936282235)]
    public async Task UsingLightIntervalSwitcher_GivenLimit_WhenGivenSteps_ThenSwitchLights(string file, bool useLimit, long expectedCount)
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync(file);

        // Act
        var lightsOn = new LightIntervalSwitcher(lines).GetOnLights(useLimit);

        // Assert
        Assert.Equal(expectedCount, lightsOn);
    }

    [Theory]
    [InlineData("../../../../../data/day22_data_test1.txt", true, 590784)]
    public async Task UsingLightSwitcher_GivenLimit_WhenGivenSteps_ThenSwitchLights(string file, bool useLimit, long expectedCount)
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync(file);
        
        // Act
        var lightsOn = new LightSwitcher(lines).GetOnLights(useLimit);

        // Assert
        Assert.Equal(expectedCount, lightsOn);
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
        var lightsOn = new LightSwitcher(steps).GetOnLights(true);

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
        var instruction = LightSwitcher.Create(line);

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