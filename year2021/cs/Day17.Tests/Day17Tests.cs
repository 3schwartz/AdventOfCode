using Xunit;

namespace Day17.Tests;

public class Day17Tests
{
    [Fact]
    public void WhenLoadData_ThenCreateCorrectTargetWindow()
    {
        // Arrange
        var input = "target area: x=20..30, y=-10..-5";

        // Act
        Launcher launcher = new Launcher(input);

        // Assert
        Assert.Equal(20, launcher.xMin);
        Assert.Equal(30, launcher.xMax);
        Assert.Equal(-10, launcher.yMin);
        Assert.Equal(-5, launcher.yMax);
    }

    [Fact]
    public void WhenCalculateMaxShot_ThenReturnMaxShot()
    {
        // Arrange
        var input = "target area: x=20..30, y=-10..-5";
        var launcher = new Launcher(input);

        // Act
        var maxVelocity = launcher.GetMaxHorizontalVelocity();

        // Assert
        Assert.True(maxVelocity.Found);
        Assert.Equal(6, maxVelocity.xVelocity);
        Assert.Equal(9, maxVelocity.yVelocity);
        Assert.Equal(45, maxVelocity.yMaxHeight);
    }

    [Fact]
    public void WhenFindDistinctVelocities_ThenReturnCorrectCount()
    {
        // Arrange
        var input = "target area: x=20..30, y=-10..-5";
        var launcher = new Launcher(input);

        // Act
        var count = launcher.GetVelocitiesCount();

        // Assert
        Assert.Equal(112, count);
    }

}