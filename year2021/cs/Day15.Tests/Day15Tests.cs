using Common;
using Xunit;

namespace Day15.Tests;

public class Day15Tests
{
    [Fact]
    public void WhenUsingDijkStras_ThenFindShortest()
    {
        // Arrange
        var dijkstras = new DijkstraFinder();
        var data = DataLoader.GetData("../../../../../data/day15_data_test.txt");

        // Act
        var nodes = dijkstras.CreateNodes(data);
        var minimum = dijkstras.FindShortest(nodes);

        // Assert
        Assert.Equal(40, minimum);
    }

    [Fact]
    public void WhenFindShortest_ThenReturnMinimum()
    {
        // Arrange
        var pathFinder = new PathFinder();
        var data = DataLoader.GetData("../../../../../data/day15_data_test.txt");

        // Act
        int minimum = pathFinder.FindShortestPath(data);

        // Assert
        Assert.Equal(40, minimum);
    }
}