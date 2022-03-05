using Xunit;

namespace Day1.Tests;

public class Tests
{
    [Fact]
    public void SplitCoordinate()
    {
        // Arrange
        var coordinates = "1,2\r\n1,1";

        // Act
        var coords = CoordinateFactory.CreateCoordinates(coordinates);

        // Assert
        Assert.True(coords.Length == 2);
        Assert.Equal(1, coords[0].X);
    }

    [Fact]
    public void PopulateGrid()
    {
        // Arrange
        var coordinates = new Coordinate[] {new(1, 1), new(0, 1)};
        var grid = new Grid(2,2);

        // Act
        grid.PopulateGrid(coordinates);

        // Assert
        Assert.Equal(0,grid.GetGrid()[0][0]);
        Assert.Equal(1, grid.GetGrid()[1][1]);
        Assert.Equal(1, grid.GetGrid()[1][0]);
    }

    [Fact]
    public void XFoldGrid()
    {
        // Arrange
        var coordinates = new Coordinate[] { new(1, 1), new(0, 1), new (2,0) };
        var grid = new Grid(3, 2);
        grid.PopulateGrid(coordinates);
        var fold = new Fold(FoldType.X, 1);

        // Act
        grid.Fold(fold);

        // Assert
        var ints = grid.GetGrid();
        Assert.Equal(2, ints.Length);
        Assert.Equal(1, ints[0].Length);
        Assert.Equal(1, ints[0][0]);
    }

    [Fact]
    public void YFoldGrid()
    {
        // Arrange
        var coordinates = new Coordinate[] { new(1, 1), new(0, 1), new(0, 2) };
        var grid = new Grid(3, 3);
        grid.PopulateGrid(coordinates);
        var fold = new Fold(FoldType.Y, 1);

        // Act
        grid.Fold(fold);

        // Assert
        var ints = grid.GetGrid();
        Assert.Equal(1, ints.Length);
        Assert.Equal(3, ints[0].Length);
        Assert.Equal(1, ints[0][0]);
    }

    [Fact]
    public void NumberOfDots()
    {
        // Arrange
        var coordinates = new Coordinate[] { new(1, 1), new(0, 1), new(0, 2) };
        var grid = new Grid(3, 3);
        grid.PopulateGrid(coordinates);

        // Act
        var numberOfDots = grid.GetNumberOfDots();

        // Assert
        Assert.Equal(3, numberOfDots);

    }
}
