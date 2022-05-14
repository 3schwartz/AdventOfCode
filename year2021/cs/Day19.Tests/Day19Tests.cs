using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Xunit;

namespace Day19.Tests;

public class Day19Tests
{
    [Fact]
    public async Task WhenCreate_ThenExactNumberOfScanners()
    {
        // Arrange
        var scannersText = await File.ReadAllTextAsync("../../../../../data/day19_data_test.txt");

        // Act
        var scanners = Scanner.CreateScanners(scannersText);

        // Assert
        Assert.Equal(5, scanners.Count);
    }

    [Fact]
    public async Task WhenGetRotations_ThenReturnCorrectAmount()
    {
        // Arrange
        var scannersText = await File.ReadAllTextAsync("../../../../../data/day19_data_test.txt");
        var scanner = Scanner.CreateScanners(scannersText)[0];

        // Act
        var rotations = scanner.Rotations;

        // Assert
        Assert.Equal(24, rotations.Count);
    }

    [Theory]
    [InlineData(38, 1)]
    [InlineData(79, 4)]
    public async Task WhenFindBeacons_ThenReturnIntersections(int expected, int scannersCount)
    {
        // Arrange
        var scannersText = await File.ReadAllTextAsync("../../../../../data/day19_data_test.txt");
        var scanners = Scanner.CreateScanners(scannersText)
            .Where((value, idx) => idx >= 0 && idx <= scannersCount)
            .ToList();

        // Act
        var beacons = Scanner.FindBeacons(scanners);

        // Assert
        Assert.Equal(expected, beacons.Beacons.Count);
    }

    [Fact]
    public async Task WhenCalculateLargestManhattenDistance_ThenReturnMax()
    {
        // Arrange
        var scannersText = await File.ReadAllTextAsync("../../../../../data/day19_data_test.txt");
        var scanners = Scanner.CreateScanners(scannersText);
        var beacons = Scanner.FindBeacons(scanners);

        // Act
        int distance = Scanner.GetLargestManhattenDistance(beacons.ScannerPositions);

        // Assert
        Assert.Equal(3621, distance);
    }
}