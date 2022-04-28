using Common;
using Xunit;

namespace Day15.Tests;

public class Day15Tests
{
    [Theory]
    [InlineData(1, 40)]
    [InlineData(5, 315)]
    public void WhenUsingPriorityQueue_ThenFindShortest(int numberOfTiles, int expected)
    {
        // Arrange
        var priorityQueueFinder = new PriorityQueueFinder();
        var data = DataLoader.GetData("../../../../../data/day15_data_test.txt");

        // Act
        var nodes = priorityQueueFinder.CreateNodes(data, numberOfTiles);
        var minimum = priorityQueueFinder.FindShortest(nodes);

        // Assert
        Assert.Equal(expected, minimum);
    }

    [Theory]
    [InlineData(1, 40)]
    [InlineData(5, 315)]
    public void WhenUsingDijkstras_ThenFindShortest(int numberOfTiles, int expected)
    {
        // Arrange
        var dijkstraFinder = new DijkstraFinder();
        var data = DataLoader.GetData("../../../../../data/day15_data_test.txt");

        // Act
        var nodes = dijkstraFinder.CreateNodes(data, numberOfTiles);
        var minimum = dijkstraFinder.FindShortest(nodes);

        // Assert
        Assert.Equal(expected, minimum);
    }

    [Theory]
    [InlineData(2, 2, 9)]
    [InlineData(5, 2, 1)]
    [InlineData(5, 5, 2)]
    public void WhenCalculateNodeTileCost_ThenCalculateCorrect(
        int idx, int jdx, int expected)
    {
        // Arrange
        var data = new string[] {"123", "456", "789"};
        var priorityQueueFinder = new PriorityQueueFinder();

        // Act
        var actual = new PriorityQueueFinder.Node(data, idx, jdx, data.Length, data[0].Length);

        // Assert
        Assert.Equal(expected, actual.Cost);
    }
}