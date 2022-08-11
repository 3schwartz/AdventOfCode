using Xunit.Abstractions;

namespace Day12.Tests;

public class Day12Tests
{
    private readonly ITestOutputHelper output;

    public Day12Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Part1()
    {
        var lines = File.ReadAllLines("../../../../../data/day12_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        simulator.TakeSteps(1_000);
        var totalEnergy = simulator.GetTotalEnergy();

        output.WriteLine($"Part 1: {totalEnergy}");
        Assert.Equal(10845, totalEnergy);
    }
    
    [Fact]
    public void WhenGivenInputCoordinate_ThenCreatePlanetCorrect()
    {
        // Arrange
        var input = "<x=-1, y=0, z=2>";

        // Act
        var moons = Moon.CreateMoon(input);

        // Assert
        Assert.NotNull(moons.Moon);
        Assert.Equal(moons.Moon!.Coordinates, new Coordinates(-1, 0, 2));
    }

    [Fact]
    public void WhenTakeAStep_ThenChangeVelocityCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        // Act
        simulator.TakeSteps(1);

        // Assert
        var idx = 0;
        foreach(var moon in simulator.Moons)
        {
            switch (idx)
            {
                case 0:
                    Assert.Equal(moon.Velocity, new Velocity(3, -1, -1));
                    break;
                case 1:
                    Assert.Equal(moon.Velocity, new Velocity(1, 3, 3));
                    break;
                case 2:
                    Assert.Equal(moon.Velocity, new Velocity(-3, 1, -3));
                    break;
                case 3:
                    Assert.Equal(moon.Velocity, new Velocity(-1, -3, 1));
                    break;
            }
            idx++;
        }
    }


    [Fact]
    public void WhenTake10Step_ThenChangeVelocityCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons);

        // Act
        simulator.TakeSteps(10);

        // Assert
        var idx = 0;
        foreach (var moon in simulator.Moons)
        {
            switch (idx)
            {
                case 0:
                    Assert.Equal(moon.Velocity, new Velocity(-3, -2, 1));
                    Assert.Equal(moon.Coordinates, new Coordinates(2,1,-3));
                    break;
                case 1:
                    Assert.Equal(moon.Velocity, new Velocity(-1, 1, 3));
                    Assert.Equal(moon.Coordinates, new Coordinates(1,-8,0));
                    break;
                case 2:
                    Assert.Equal(moon.Velocity, new Velocity(3, 2, -3));
                    Assert.Equal(moon.Coordinates, new Coordinates(3,-6,1));
                    break;
                case 3:
                    Assert.Equal(moon.Velocity, new Velocity(1, -1, -1));
                    Assert.Equal(moon.Coordinates, new Coordinates(2,0,4));
                    break;
            }
            idx++;
        }
    }

    [Fact]
    public void WhenTake10Steps_ThenCorrectTotalEnergy()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons);

        // Act
        simulator.TakeSteps(10);
        int totalEnergy = simulator.GetTotalEnergy();

        // Assert
        Assert.Equal(totalEnergy, 179);

    }
}

internal readonly record struct Coordinates(int X, int Y, int Z);
internal readonly record struct Velocity(int X, int Y, int Z);

internal class Moon
{
    internal Coordinates Coordinates { get; private set; }
    internal Velocity Velocity { get; private set; }

    private Moon(Coordinates coordinates)
    {
        Coordinates = coordinates;
    }

    internal static (Moon? Moon, Exception? Error) CreateMoon(string coordinates)
    {
        var coords = coordinates.Trim('<', '>')
            .Split(", ")
            .SelectMany(n => n.Split("=")
                .Select((v, i) => new { v, i })
                .Where(a => a.i == 1)
                .Select(a => a.v))
            .Select(n => int.Parse(n.ToString()))
            .ToArray();
        if (coords.Length != 3)
        {
            return (null, new Exception($"Coordinates not able to be parsed: {coords}"));
        }
        return (new Moon(new Coordinates(coords[0], coords[1], coords[2])), null);
    }

    internal Velocity FindVelocityFromMoon(Moon moon)
    {
        var x = FindPullDirection(Coordinates.X, moon.Coordinates.X);
        var y = FindPullDirection(Coordinates.Y, moon.Coordinates.Y);
        var z = FindPullDirection(Coordinates.Z, moon.Coordinates.Z);
        return new Velocity(x, y, z);
    }

    private static int FindPullDirection(int current, int other)
    {
        if (current > other)
        {
            return -1;
        }
        if (other > current)
        {
            return 1;
        }
        return 0;
    }

    internal void ApplyVelocity(Velocity pull)
    {
        Velocity = new Velocity(Velocity.X + pull.X, Velocity.Y + pull.Y, Velocity.Z + pull.Z);
        Move();
    }

    private void Move()
    {
        Coordinates = new Coordinates(Coordinates.X + Velocity.X, Coordinates.Y + Velocity.Y, Coordinates.Z + Velocity.Z);
    }
}

internal class MoonSimulator
{
    private readonly List<Moon> moons;
    internal IReadOnlyCollection<Moon> Moons => moons.ToList();
    internal int Steps { get;private set; }

    public MoonSimulator(List<Moon> moons)
    {
        this.moons = moons;
        Steps = 0;
    }

    internal void TakeSteps(int v)
    {
        var steps = 0;
        while(steps < v)
        {
            var velocities = new Dictionary<int, IList<Velocity>>();
            for (int i = 0; i < moons.Count; i++)
            {
                velocities[i] = new List<Velocity>();
                for (var j = 0; j < moons.Count; j++)
                {
                    if (i == j) continue;
                    velocities[i].Add(moons[i].FindVelocityFromMoon(moons[j]));
                }
            }
            foreach (var (key, pulls) in velocities)
            {
                var pull = pulls.Aggregate((b, c) => new Velocity(b.X + c.X, b.Y + c.Y, b.Z + c.Z));
                moons[key].ApplyVelocity(pull);
            }

            steps++;
            Steps++;
        }
    }

    internal int GetTotalEnergy()
    {
        var totalEnergy = 0;
        foreach(var moon in moons)
        {
            var potentialEnergy = Math.Abs(moon.Coordinates.X) + Math.Abs(moon.Coordinates.Y) + Math.Abs(moon.Coordinates.Z);
            var kineticEnergy = Math.Abs(moon.Velocity.X) + Math.Abs(moon.Velocity.Y) + Math.Abs(moon.Velocity.Z);
            totalEnergy += potentialEnergy * kineticEnergy;
        }
        return totalEnergy;
    }
}