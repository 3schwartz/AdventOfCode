using Xunit;

namespace Day21.Tests;

public class Day21Tests
{
    [Fact]
    public void WhenGameStarts_ThenFindWinner()
    {
        // Arrange
        var p1 = new Player("Player 1 starting position: 4");
        var p2 = new Player("Player 2 starting position: 8");
        var game = new Game(p1, p2);
        
        // Act
        var result = game.Start();

        // Assert
        Assert.Equal(1000, result.Winner.Score);
        Assert.Equal(745, result.Looser.Score);
        Assert.Equal(993, result.DieRolled);
        Assert.Equal(739785, result.GameResult);
    }

    [Fact]
    public void WhenPlayWithDiracDice_ThenFindPlayerWins()
    {
        // Arrange
        var p1 = new Player("Player 1 starting position: 4");
        var p2 = new Player("Player 2 starting position: 8");
        var game = new DiracGame(p1, p2);

        // Act
        var result = game.Start();

        // Assert
        Assert.Equal(444356092776315, result.Item1);
        Assert.Equal(341960390180808, result.Item2);
    }
}
