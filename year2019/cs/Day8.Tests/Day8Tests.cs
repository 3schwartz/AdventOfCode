using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Xunit;
using Xunit.Abstractions;

namespace Day8.Tests;

public class Day8Tests
{
    private readonly ITestOutputHelper output;

    public Day8Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Part2()
    {
        var input = File.ReadAllText("../../../../../data/day8_data.txt");
        var columns = 25;
        var rows = 6;
        var imageSize = rows * columns;
        var image = new Dictionary<(int, int), char>();
        for (int i = 0; i < input.Length; i++)
        {
            var row = i / columns - rows * (i / imageSize);
            var col = i % columns;
            if (input[i] != '2')
            {
                image.TryAdd((row, col), input[i]);
            }
        }

        Span<char> rowsInput = stackalloc char[columns];
        for (int i = 0; i < rows; i++)
        {
            for (int j = 0; j < columns; j++)
            {
                rowsInput[j] = image[(i, j)] == '1' ? '#' : '.';
            }
            output.WriteLine(rowsInput.ToString());
        }
    }

    [Fact]
    public void Program()
    {
        // Arrange
        var input = File.ReadAllText("../../../../../data/day8_data.txt");
        var imageSize = 25 * 6;
        var layers = new Dictionary<int, List<int>>(input.Length / imageSize);
        for (int i = 0; i < input.Length; i++)
        {
            if(!layers.TryGetValue(i / imageSize, out var layer))
            {
                layer = new List<int>(imageSize);
                layers[i / imageSize] = layer;
            }
            layer.Add(input[i] - '0');
        }

        var minZeroDigit = int.MaxValue;
        var minZeroDigitIdx = int.MaxValue;
        for (int i = 0; i < layers.Count; i++)
        {
            var zeroDigits = 0;
            foreach(var pixel in layers[i])
            {
                if(pixel == 0)
                {
                    zeroDigits++;
                }
            }
            if(zeroDigits < minZeroDigit)
            {
                minZeroDigit = zeroDigits;
                minZeroDigitIdx = i;
            }
        }

        var oneDigits = 0;
        var secondDigits = 0;
        foreach (var pixel in layers[minZeroDigitIdx])
        {
            if(pixel == 1)
            {
                oneDigits++;
            }
            if (pixel == 2)
            {
                secondDigits++;
            }
        }

        output.WriteLine($"Part 1: {oneDigits * secondDigits}");        
    }
}