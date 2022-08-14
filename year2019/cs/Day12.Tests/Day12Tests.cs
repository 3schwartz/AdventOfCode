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
    public void Part2()
    {
        var lines = File.ReadAllLines("../../../../../data/day12_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        long stepsToInitial = simulator.StepsToGetBackToInitial();

        output.WriteLine($"Part 2: {stepsToInitial}");
        Assert.Equal(551272644867044, stepsToInitial);
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
    public void WhenFindStepsBeforeBackToInitialPLinq_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test2_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        // Act
        long stepsToInitial = simulator.StepsToGetBackToInitialPLinq();

        // Assert
        Assert.Equal(4686774924, stepsToInitial);
    }

    [Fact]
    public void WhenFindStepsBeforeBackToInitialParralel_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test2_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        // Act
        long stepsToInitial = simulator.StepsToGetBackToInitialParallel();

        // Assert
        Assert.Equal(4686774924, stepsToInitial);
    }

    [Fact]
    public async Task WhenFindStepsBeforeBackToInitialAsync_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test2_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        // Act
        long stepsToInitial = await simulator.StepsToGetBackToInitialAsync();

        // Assert
        Assert.Equal(4686774924, stepsToInitial);
    }

    [Fact]
    public void WhenFindStepsBeforeBackToInitial_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test2_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

        // Act
        long stepsToInitial = simulator.StepsToGetBackToInitial();

        // Assert
        Assert.Equal(4686774924, stepsToInitial);
    }

    [Fact]
    public void WhenTake10Step_ThenChangeVelocityCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day12_test_data.txt");
        var moons = lines.Select(line => Moon.CreateMoon(line).Moon).ToList();
        var simulator = new MoonSimulator(moons!);

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
        var simulator = new MoonSimulator(moons!);

        // Act
        simulator.TakeSteps(10);
        int totalEnergy = simulator.GetTotalEnergy();

        // Assert
        Assert.Equal(179, totalEnergy);
    }
}