namespace Day10.Tests;

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