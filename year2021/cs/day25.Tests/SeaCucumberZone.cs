using System;
using System.Collections.Generic;

namespace day25.Tests;

internal class SeaCucumberZone
{
    public int Steps { get; private set; }
    private readonly IDictionary<(int, int), char> map;
    private readonly int maxY;
    private readonly int maxX;

    public SeaCucumberZone(string[] lines)
    {
        map = new Dictionary<(int, int), char>();
        int i;
        for (i = 0; i < lines.Length; i++)
        {
            int j;
            for (j = 0; j < lines[i].Length; j++)
            {
                if(lines[i][j] == '>' || lines[i][j] == 'v')
                {
                    map.Add((i,j), lines[i][j]);
                }
            }
            maxX = j;
        }
        maxY = i;
    }

    internal void MoveToSteadyState()
    {
        int totalSteps = 0;
        int stepsTaken;
        do
        {
            stepsTaken = 0;
            stepsTaken += MoveSide();
            stepsTaken += MoveDown();

            totalSteps++;
        } while (stepsTaken > 0);
        Steps = totalSteps;
    }

    private int MoveSide()
    {
        return Move('>', (cucumber) => (cucumber.Item1, (cucumber.Item2 + 1) % maxX));
    }

    private int MoveDown()
    {
        return Move('v', (cucumber) => ((cucumber.Item1 + 1) % maxY, cucumber.Item2));
    }

    private int Move(char direction, Func<(int,int), (int,int)> getTo)
    {
        var moves = new List<((int, int) From, (int, int) To)>();

        foreach (var cucumber in map)
        {
            if (cucumber.Value != direction) continue;

            var to = getTo(cucumber.Key);
            if (!map.ContainsKey(to))
            {
                moves.Add((cucumber.Key, to));
            }
        }

        foreach (var move in moves)
        {
            map.Add(move.To, direction);
            map.Remove(move.From);
        }

        return moves.Count;
    }
}