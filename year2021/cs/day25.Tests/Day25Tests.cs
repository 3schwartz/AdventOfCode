using System.IO;
using System.Threading.Tasks;
using Xunit;

namespace day25.Tests;

public class Day25Tests
{
    [Fact]
    public async Task WhenMovingSeaCucumbersToSteadyState_ThenCorrectStepsAsync()
    {
        // Arrange
        var lines = await File.ReadAllLinesAsync("../../../../../data/day25_data_test.txt");
        SeaCucumberZone cucumber = new SeaCucumberZone(lines);

        // Act
        cucumber.MoveToSteadyState();

        // Assert
        Assert.Equal(58, cucumber.Steps);
    }
}
