using System.IO;
using System.Threading.Tasks;
using Xunit;

namespace Day20.Tests;

public class Day20Tests
{
    [Theory]
    [InlineData(0, 10, "../../../../../data/day20_data_test.txt")]
    [InlineData(1, 24, "../../../../../data/day20_data_test.txt")]
    [InlineData(2, 35, "../../../../../data/day20_data_test.txt")]
    [InlineData(2, 5432, "../../../../../data/day20_data.txt")]
    [InlineData(50, 16016, "../../../../../data/day20_data.txt")]
    public async Task GivenImage_WhenEnhance_ThenCorrectPixelCount(int times, int expected, string file)
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync(file);
        var image = new Image(lines);

        // Act
        image.Enhance(times);
        var pixelCount = image.GetPixelCount();

        // Assert
        Assert.Equal(expected, pixelCount);
    }

    [Theory]
    [InlineData(0, 10, "../../../../../data/day20_data_test.txt")]
    [InlineData(1, 24, "../../../../../data/day20_data_test.txt")]
    [InlineData(2, 35, "../../../../../data/day20_data_test.txt")]
    [InlineData(2, 5432, "../../../../../data/day20_data.txt")]
    [InlineData(50, 16016, "../../../../../data/day20_data.txt")]
    public async Task GivenImageSet_WhenEnhance_ThenCorrectPixelCount(int times, int expected, string file)
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync(file);
        var image = new ImageSet(lines);

        // Act
        image.Enhance(times);
        var pixelCount = image.GetPixelCount();

        // Assert
        Assert.Equal(expected, pixelCount);
    }

}