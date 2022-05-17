using System;
using System.IO;
using System.Numerics;
using System.Threading.Tasks;
using Xunit;

namespace Day20.Tests;

public class Day20Tests
{
    [Fact]
    public async Task Test1Async()
    {
        var lines = await File.ReadAllLinesAsync("../../../../../data/day20_data_test.txt");
    }

    [Fact]
    public void CreateBinaryFromSpan()
    {
        Span<int> ints = stackalloc int[9] {0, 0, 0, 0, 0, 0, 0, 1, 1};
        var value = 0;

        for (var i = 0; i < ints.Length; i++)
        {
            if (ints[^(i + 1)] == 1)
            {
                value += Image.Power(2,i);
            }
        }
    }

}