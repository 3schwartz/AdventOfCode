using System;
using System.IO;
using Xunit;
using System.Linq;
using System.Collections.Generic;
using Xunit.Abstractions;

namespace Day5.Tests;

public class Day5Tests
{
    private readonly ITestOutputHelper output;

    public Day5Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Program()
    {
        var codes = File.ReadAllText("../../../../../data/day5_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();

        int diagnostocCode = coder.RunTest(codes, 5);

        output.WriteLine($"Part 1 : {diagnostocCode}");
    }

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

    [Fact]
    public void WhenRunningTest_ThenCorrect()
    {
        // Arrange
        var codes = File.ReadAllText("../../../../../data/day5_data.txt")
            .Split(",")
            .Select(c => int.Parse(c))
            .ToList();
        var coder = new IntCoder();

        // Act
        int diagnostocCode = coder.RunTest(codes, 1);

        // Assert
        Assert.Equal(13087969, diagnostocCode);
    }

    internal class IntCoder
    {
        internal int RunTest(IList<int> codes, int input)
        {
            var idx = 0;
            var outputs = new List<int>();
            do
            {
                var execution = codes[idx];
                switch (execution % 100)
                {
                    case 1:
                        codes[GetIdxFromMode(codes, execution, 3, idx)] = 
                            codes[GetIdxFromMode(codes, execution, 2, idx)] + codes[GetIdxFromMode(codes, execution, 1, idx)];
                        idx += 4;
                        break;
                    case 2:
                        codes[GetIdxFromMode(codes, execution, 3, idx)] =
                            codes[GetIdxFromMode(codes, execution, 2, idx)] * codes[GetIdxFromMode(codes, execution, 1, idx)];
                        idx += 4;
                        break;
                    case 3:
                        codes[GetIdxFromMode(codes, execution, 1, idx)] = input;
                        idx += 2;
                        break;
                    case 4:
                        outputs.Add(codes[GetIdxFromMode(codes, execution, 1, idx)]);
                        idx += 2;
                        break;
                    case 5:
                        if(codes[GetIdxFromMode(codes,execution,1, idx)] != 0)
                        {
                            idx = codes[GetIdxFromMode(codes, execution, 2, idx)];
                            break;
                        }
                        idx += 3;
                        break;
                    case 6:
                        if (codes[GetIdxFromMode(codes, execution, 1, idx)] == 0)
                        {
                            idx = codes[GetIdxFromMode(codes, execution, 2, idx)];
                            break;
                        }
                        idx += 3;
                        break;
                    case 7:
                        codes[GetIdxFromMode(codes, execution, 3, idx)] =
                            codes[GetIdxFromMode(codes, execution, 1, idx)] < codes[GetIdxFromMode(codes, execution, 2, idx)] ?
                            1 : 0;
                        idx += 4;
                        break;
                    case 8:
                        codes[GetIdxFromMode(codes, execution, 3, idx)] =
                            codes[GetIdxFromMode(codes, execution, 1, idx)] == codes[GetIdxFromMode(codes, execution, 2, idx)] ?
                            1 : 0;
                        idx += 4;
                        break;
                    case 99:
                        idx = codes.Count;
                        break;
                    default:
                        throw new Exception($"OptCode not known {execution}");
                }
            } while (idx < codes.Count);

            return outputs[^1];
        }

        private int GetIdxFromMode(IList<int> codes, int execution, int parameterPosition, int idx)
        {
            var mode = execution / ((int)Math.Pow(10, 1 + parameterPosition));
            mode %= 10;
            return mode == 1 ? idx + parameterPosition : codes[idx + parameterPosition];

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

        internal int FindOutput(IList<int> codes)
        {
            for (int i = 0; i <= 99; i++)
            {
                for (int j = 0; j <= 99; j++)
                {
                    var copy = new List<int>(codes);
                    RunInstructions(copy, i, j);
                    if (copy[0] == 19690720)
                    {
                        return 100 * i + j;
                    }
                }
            }
            throw new Exception("Nothing found");
        }
    }
}