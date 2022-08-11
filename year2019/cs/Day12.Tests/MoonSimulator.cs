﻿namespace Day12.Tests;

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

    internal long StepsToGetBackToInitial()
    {
        var steps = new List<int>();
        var directions = new List<Moon.Direction>()
        {
            Moon.Direction.X,Moon.Direction.Y,Moon.Direction.Z,
        };
        foreach (var direction in directions)
        {
            var moonSteps = FindStepsToInitialInDirection(direction);
            steps.Add(moonSteps);
        }

        var yZ = LeastCommonMultiple(steps[1], steps[2]);
        var xYZ = LeastCommonMultiple(steps[0], yZ);
        return xYZ;
    }

    private int FindStepsToInitialInDirection(Moon.Direction direction)
    {
        var initialPositions = moons.Select(m => m.GetDirectionPosition(direction)).ToList();
        var moonSteps = 0;
        var velocities = new Dictionary<int, IList<int>>();
        while (true)
        {
            for (int i = 0; i < moons.Count; i++)
            {
                velocities[i] = new List<int>();
                for (var j = 0; j < moons.Count; j++)
                {
                    if (i == j) continue;
                    velocities[i].Add(moons[i].FindVelocityFromMoonInDirection(moons[j], direction));
                }
            }
            foreach (var (key, pulls) in velocities)
            {
                var pull = pulls.Sum();
                moons[key].ApplyVelocityInDirection(pull, direction);
            }
            moonSteps++;

            if (initialPositions.SequenceEqual(moons.Select(m => m.GetDirectionPosition(direction))))
            {
                break;
            }
        }
        return moonSteps;
    }

    private static long LeastCommonMultiple(long a, long b)
    {
        return a * b / GreatestCommonDivisor(a, b);
    }

    private static long GreatestCommonDivisor(long a, long b)
    {
        while (b != 0)
        {
            (a, b) = (b, a % b);
        }
        return a;
    }

    internal void TakeSteps(int v)
    {
        var steps = 0;
        var velocities = new Dictionary<int, IList<Velocity>>();
        while (steps < v)
        {
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