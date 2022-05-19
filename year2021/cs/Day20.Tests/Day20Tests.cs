using System.IO;
using System.Threading.Tasks;
using Xunit;

namespace Day20.Tests;

public class Day20Tests
{
    [Theory]
    [InlineData(0, 10)]
    [InlineData(1, 24)]
    [InlineData(2, 35)]
    public async Task GivenImage_WhenEnhance_ThenCorrectPixelCount(int times, int expected)
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync("../../../../../data/day20_data_test.txt");
        var image = new Image(lines);

        // Act
        image.Enhance(times);
        var pixelCount = image.GetPixelCount();

        // Assert
        Assert.Equal(expected, pixelCount);
    }

    [Theory]
    [InlineData(0, 10)]
    [InlineData(1, 24)]
    [InlineData(2, 35)]
    public async Task GivenImageSet_WhenEnhance_ThenCorrectPixelCount(int times, int expected)
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync("../../../../../data/day20_data_test.txt");
        var image = new ImageSet(lines);

        // Act
        image.Enhance(times);
        var pixelCount = image.GetPixelCount();

        // Assert
        Assert.Equal(expected, pixelCount);
    }

}