using System;
using System.IO;
using System.Numerics;
using System.Threading.Tasks;
using Xunit;

namespace Day20.Tests;

public partial class Day20Tests
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
                value += Power(2,i);
            }
        }
    }

    private static int Power(int value, int power)
    {
        if (power == 0)
        {
            return 1;
        }

        var final = 1;
        while (power > 0)
        {
            final *= value;
            power--;
        }

        return final;
    }

}