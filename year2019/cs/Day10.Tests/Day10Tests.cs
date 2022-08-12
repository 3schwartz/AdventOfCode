using Shouldly;

namespace Day10.Tests;

public class Day10Tests
{
    [Theory]
    // Part 1
    [InlineData("", 19, 14, 274)]
    [InlineData("_test", 3, 4, 8)]
    [InlineData("_test2", 11, 13, 210)]

    public async Task WhenGivenMap_ThenCorrectDetectedAsteroidsAsync(string file, int x, int y, int expected)
    {
        // Arrange
        var data = File.ReadAllText($"../../../../../data/day10{file}_data.txt");
        var asteroidMap = new AsteroidMap(data);
        var monitoringStation = new MonitoringStation(asteroidMap);

        // Act
        var monitoringLocation = await monitoringStation.FindLocationWithMaxDetectedAsteroidsAsync();

        // Assert
        monitoringLocation.Coordinate.X.ShouldBe(x);
        monitoringLocation.Coordinate.Y.ShouldBe(y);
        monitoringLocation.DetectedAsteroids.ShouldBe(expected);
    }

    [Theory]
    // Part 1
    [InlineData("", 19, 14, 274)]
    [InlineData("_test", 3,4,8)]
    [InlineData("_test2", 11, 13, 210)]

    public void WhenGivenMap_ThenCorrectDetectedAsteroids(string file, int x, int y, int expected)
    {
        // Arrange
        var data = File.ReadAllText($"../../../../../data/day10{file}_data.txt");
        var asteroidMap = new AsteroidMap(data);
        var monitoringStation = new MonitoringStation(asteroidMap);

        // Act
        var monitoringLocation = monitoringStation.FindLocationWithMaxDetectedAsteroidsNew();

        // Assert
        monitoringLocation.Coordinate.X.ShouldBe(x);
        monitoringLocation.Coordinate.Y.ShouldBe(y);
        monitoringLocation.DetectedAsteroids.ShouldBe(expected);
    }

    [Theory]
    // Part 2
    [InlineData("", 19, 14, 305)]
    [InlineData("_test2", 11, 13, 802)]
    public void WhenGivenLocation_ThenFindCorrectOrderOfVaporize(string file, int x, int y, int expected)
    {
        var data = File.ReadAllText($"../../../../../data/day10{file}_data.txt");
        var asteroidMap = new AsteroidMap(data);
        var monitoringStation = new MonitoringStation(asteroidMap);

        // Act
        IList<(int X, int Y)> vaporized = monitoringStation.VaporizeAsteroids((x, y));
        var actual = vaporized[199].X * 100 + vaporized[199].Y;

        // Assert
        actual.ShouldBe(expected);
    }

}