using System.Collections.Generic;
using System.IO;
using Xunit;
using Xunit.Abstractions;

namespace Day6.Tests;

public class Day6Tests
{
    private readonly ITestOutputHelper output;

    public Day6Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Program()
    {
        var lines = File.ReadAllLines("../../../../../data/day6_data.txt");
        var orbitMap = new OrbitMap(lines);

        int orbits = orbitMap.GetOrbitCount();

        output.WriteLine($"Part 1: {orbits}");
    }

    [Fact]
    public void WhenGetOrbitAmount_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllLines("../../../../../data/day6_data_test.txt");
        var orbitMap = new OrbitMap(lines);

        // Act
        int orbits = orbitMap.GetOrbitCount();

        // Assert
        Assert.Equal(42, orbits);
    }
}

internal class OrbitMap
{
    private readonly IDictionary<string, IList<string>> orbits;

    public OrbitMap(string[] lines)
    {
        orbits = new Dictionary<string, IList<string>>();
        foreach(var line in lines)
        {
            var split = line.Split(")");
            AddOrUpdate(split[0], split[1]);
        }
    }

    internal int GetOrbitCount()
    {
        return GetAroundOrbits("COM", 1);
    }

    private int GetAroundOrbits(string center, int debt)
    {
        if(orbits.TryGetValue(center, out var around))
        {
            var sum = around.Count * debt;
            foreach(var o in around)
            {
                sum += GetAroundOrbits(o, debt + 1);
            }
            return sum;
        }
        return 0;
    }

    private void AddOrUpdate(string center, string orbit)
    {
        if (orbits.ContainsKey(center))
        {
            orbits[center].Add(orbit);
            return;
        }
        orbits[center] = new List<string> { orbit };
    }
}