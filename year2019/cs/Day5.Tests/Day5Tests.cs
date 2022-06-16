using System;
using System.IO;
using Xunit;
using System.Linq;
using System.Collections.Generic;

namespace Day5.Tests;

public class Day5Tests
{
    [Fact]
    public void WhenRunningInitialIntCodeProgram_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllText("../../../../../data/day2_data.txt").Split(",");
        var coder = new IntCoder();
        var codes = lines.Select(c => int.Parse(c)).ToList();

        // Act
        coder.RunInstructions(codes, 12, 2);

        // Assert
        Assert.Equal(5305097, codes[0]);
    }

    [Fact]
    public void WhenFindingOutput_ThenCorrect()
    {
        // Arrange
        var lines = File.ReadAllText("../../../../../data/day2_data.txt").Split(",");
        var coder = new IntCoder();
        var codes = lines.Select(c => int.Parse(c)).ToList();

        // Act
        var optimal = coder.FindOutput(codes);

        // Assert
        Assert.Equal(4925, optimal);
    }

    internal class IntCoder
    {
        internal int FindOutput(IList<int> codes)
        {
            for (int i = 0; i <= 99; i++)
            {
                for (int j = 0; j <= 99; j++)
                {
                    var copy = new List<int>(codes);
                    RunInstructions(copy, i, j);
                    if(copy[0] == 19690720)
                    {
                        return 100 * i + j;
                    }
                }
            }
            throw new Exception("Nothing found");
        }

        internal void RunInstructions(IList<int> codes, int noun, int verb)
        {
            codes[1] = noun;
            codes[2] = verb;
            var idx = 0;
            do
            {
                var optcode = codes[idx];
                switch (optcode)
                {
                    case 1:
                        codes[codes[idx + 3]] = codes[codes[idx + 2]] + codes[codes[idx + 1]];
                        break;
                    case 2:
                        codes[codes[idx + 3]] = codes[codes[idx + 2]] * codes[codes[idx + 1]];
                        break;
                    case 99:
                        idx = codes.Count;
                        break;
                    default:
                        throw new Exception($"OptCode not known {optcode}");
                }
                idx += 4;
            } while (idx < codes.Count);
        }
    }
}