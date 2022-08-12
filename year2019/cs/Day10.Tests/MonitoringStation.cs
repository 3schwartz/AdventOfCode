namespace Day10.Tests;

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

    internal async Task<MonitoringLocation> FindLocationWithMaxDetectedAsteroidsAsync()
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
        var finished = await Task.WhenAll(tasks);

        return finished.Select(t => t).MaxBy((m) => m.DetectedAsteroids);
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
