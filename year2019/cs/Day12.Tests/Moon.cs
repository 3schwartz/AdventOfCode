namespace Day12.Tests;

public class Moon
{
    internal enum Direction
    {
        X, Y, Z
    }
    private int cX = 0;
    private int cY = 0;
    private int cZ = 0;

    internal Coordinates Coordinates => new(cX, cY, cZ);
    
    private int vX = 0;
    private int vY = 0;
    private int vZ = 0;

    internal Velocity Velocity => new(vX, vY, vZ);

    private Moon(Coordinates coordinates)
    {
        cX = coordinates.X;
        cY = coordinates.Y;
        cZ = coordinates.Z;
    }

    public static (Moon? Moon, Exception? Error) CreateMoon(string coordinates)
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
        var otherMoonCoordinates = moon.Coordinates;
        var x = FindPullDirection(cX, otherMoonCoordinates.X);
        var y = FindPullDirection(cY, otherMoonCoordinates.Y);
        var z = FindPullDirection(cZ, otherMoonCoordinates.Z);
        return new Velocity(x, y, z);
    }

    internal int FindVelocityFromMoonInDirection(Moon other, Direction direction)
    {
        return direction switch
        {
            Direction.X => FindPullDirection(cX, other.Coordinates.X),
            Direction.Y => FindPullDirection(cY, other.Coordinates.Y),
            Direction.Z => FindPullDirection(cZ, other.Coordinates.Z),
            _ => throw new NotImplementedException(),
        };
    }

    internal void ApplyVelocity(Velocity pull)
    {
        vX += pull.X;
        vY += pull.Y;
        vZ += pull.Z;
        Move();
    }

    internal void ApplyVelocityInDirection(int pull, Direction direction)
    {
        switch (direction)
        {
            case Direction.X:
                vX += pull;
                cX += vX;
                break;
            case Direction.Y:
                vY += pull;
                cY += vY;
                break;
            case Direction.Z:
                vZ += pull;
                cZ += vZ;
                break;
        }
    }

    internal DirectionPosition GetDirectionPosition(Direction diretion)
    {
        return diretion switch
        {
            Direction.X => new DirectionPosition(cX, vX),
            Direction.Y => new DirectionPosition(cY, vY),
            Direction.Z => new DirectionPosition(cZ, vZ),
            _ => throw new NotImplementedException(),
        };
    }

    internal int GetTotalEnergy()
    {
        var potentialEnergy = Math.Abs(cX) + Math.Abs(cY) + Math.Abs(cZ);
        var kineticEnergy = Math.Abs(vX) + Math.Abs(vY) + Math.Abs(vZ);
        return potentialEnergy * kineticEnergy;
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
    private void Move()
    {
        cX += vX;
        cY += vY;
        cZ += vZ;
    }
}
