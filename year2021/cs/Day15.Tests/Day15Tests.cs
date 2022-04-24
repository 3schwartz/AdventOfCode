using Common;
using System;
using Xunit;

namespace Day15.Tests;

public class Day15Tests
{
    [Fact]
    public void WhenFindShortest_ThenReturnMinimum()
    {
        // Arrange
        var data = DataLoader.GetData("../../../../../data/day15_data_test.txt");

        // Act
        int minimum = FindShortestPath(data);

        // Assert
        Assert.Equal(40, minimum);
    }

    private int FindShortestPath(string[] data)
    {
        var nRow = data.Length;
        var nCol = data[0].Length;
        var minimum = int.MaxValue;
        var i = 0;
        var j = 0;
        var path = -(data[i][j] - '0');

        return FollowPath(data, i, j, nRow, nCol, path, minimum);
    }

    private int FollowPath(string[] data, int i, int j, int nRow, int nCol, int path, int minimum)
    {
        path += (data[i][j] - '0');

        if (path > minimum)
        {
            return minimum;
        }
        if (i == nRow - 1 && j == nCol - 1)
        {
            return path;
        }

        if (j < nCol - 1)
        {
            minimum = FollowPath(data, i, j+1, nRow, nCol, path, minimum);
        }

        if (i < nRow - 1)
        {
            minimum = FollowPath(data, i+1, j, nRow, nCol, path, minimum);
        }

        return minimum;
    }
}