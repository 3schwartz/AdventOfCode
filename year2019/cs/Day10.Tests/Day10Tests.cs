using Shouldly;
using System.Collections.Concurrent;

namespace Day10.Tests;

public class Day10Tests
{
    [Theory]
    // Part 1
    [InlineData("", 19, 14, 274)]
    [InlineData("_test", 3, 4, 8)]
    [InlineData("_test2", 11, 13, 210)]

    public void WhenGivenMap_ThenCorrectDetectedAsteroidsAsync(string file, int x, int y, int expected)
    {
        // Arrange
        var data = File.ReadAllText($"../../../../../data/day10{file}_data.txt");
        var asteroidMap = new AsteroidMap(data);
        var monitoringStation = new MonitoringStation(asteroidMap);

        // Act
        var monitoringLocation = monitoringStation.FindLocationWithMaxDetectedAsteroidsAsync();

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

internal class MonitoringStation
{
    private readonly AsteroidMap asteroidMap;

    public MonitoringStation(AsteroidMap asteroidMap)
    {
        this.asteroidMap = asteroidMap;
    }

    internal MonitoringLocation FindLocationWithMaxDetectedAsteroidsNew()
    {
        var location = new MonitoringLocation(int.MinValue, (0, 0));
        
        foreach(var asteroid in asteroidMap.Asteroids)
        {
            var nearest = FindNearestAsteroids(asteroid, asteroidMap.Asteroids);

            if (nearest.Count is var count && count > location.DetectedAsteroids)
            {
                location = new MonitoringLocation(count, asteroid);
            }
        }
        return location;
    }

    internal MonitoringLocation FindLocationWithMaxDetectedAsteroidsAsync()
    {
        var count = asteroidMap.Asteroids.Count;
        var tasks = new Task<MonitoringLocation>[count];

        var idx = 0;
        foreach (var asteroid in asteroidMap.Asteroids)
        {
            tasks[idx] = Task.Run(() =>
            {
                var nearest = FindNearestAsteroids(asteroid, asteroidMap.Asteroids);
                return new MonitoringLocation(nearest.Count, asteroid);
            });

            idx++;
        }
        Task.WaitAll(tasks);

        return tasks.Select(t => t.Result).MaxBy((m) => m.DetectedAsteroids);
    }

    internal IList<(int X, int Y)> VaporizeAsteroids((int x, int y) center)
    {
        var asteroids = new HashSet<(int, int)>(asteroidMap.Asteroids);
        var vaporized = new List<(int, int)>();

        while(asteroids.Count > 1)
        {
            var angles = FindNearestAsteroids(center, asteroids);
            foreach(var coord in angles.OrderBy(k => k.Key))
            {
                vaporized.Add(coord.Value);
                asteroids.Remove(coord.Value);
            }
        }
        return vaporized;
    }

    private static IDictionary<double, (int X, int Y)> FindNearestAsteroids((int X, int Y) center, ISet<(int X, int Y)> asteroids)
    {
        var angles = new Dictionary<double, (int X, int Y)>();
        foreach(var asteroid in asteroids)
        {
            if (asteroid.X == center.X && asteroid.Y == center.Y)
            {
                continue;
            }
            var angle = FindTwelveOclockAngleFromCenter(asteroid, center);
            if(angles.TryAdd(angle, asteroid))
            {
                continue;
            }
            var previous = angles[angle];
            if (ManhattanDistance(previous, center) > ManhattanDistance(asteroid, center))
            {
                angles[angle] = asteroid;
            };
        }
        return angles;
    }

    private static int ManhattanDistance((int X, int Y) other, (int X, int Y) center)
    {
        return Math.Abs(other.X - center.X) + Math.Abs(other.Y - center.Y);
    }

    private static double FindTwelveOclockAngleFromCenter((int X, int Y) point, (int X, int Y) center)
    {
        var x = point.X - center.X;
        var y = point.Y - center.Y;
        (x, y) = (y, x);
        var angle = -Math.Atan2(y, x);
        return angle;
    }

}

internal readonly struct MonitoringLocation
{
    public int DetectedAsteroids { get; }
    public (int X,int Y) Coordinate { get; }

    public MonitoringLocation(int detectedAsteroids, (int,int) coord)
    {
        DetectedAsteroids = detectedAsteroids;
        Coordinate = coord;
    }
}

internal class AsteroidMap
{
    public ISet<(int X,int Y)> Asteroids { get; init; }

    public AsteroidMap(string data)
    {
        var lines = data.Split("\r\n");
        Asteroids = new HashSet<(int, int)>();
        for (int i = 0; i < lines.Length; i++)
        {
            var line = lines[i];
            for (int j = 0; j < line.Length; j++)
            {
                if (line[j] == '#')
                {
                    Asteroids.Add((j, i));
                }
            }

        }
    }
}