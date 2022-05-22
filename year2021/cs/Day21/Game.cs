namespace Day21;

internal class Game
{
    private readonly Player p1;
    private readonly Player p2;

    public Game(Player p1, Player p2)
    {
        this.p1 = p1;
        this.p2 = p2;
    }

    internal Result Start()
    {
        var playersTurn = p1;
        var otherPlayer = p2;
        var rolls = 0;
        var dice = 1;
        do
        {
            rolls += 3;

            var i = (dice - 1) % 100 + 1;
            var move = i + i + 1 + i + 2;

            playersTurn.Move(move);
            if (playersTurn.Score >= 1000)
            {
                return new Result(playersTurn, otherPlayer, rolls, otherPlayer.Score * rolls);
            }

            dice += 3;
            var tmp = playersTurn;
            playersTurn = otherPlayer;
            otherPlayer = tmp;
        }
        while (true);
    }

    internal record struct Result(Player Winner, Player Looser, int DieRolled, int GameResult);
}
