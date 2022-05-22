
namespace Day21
{
    internal class DiracGame
    {
        private readonly IDictionary<(int, int, int, int), (long,long)> visited;
        private readonly IList<int> diceSums;
        private Player player1;
        private Player player2;

        public DiracGame(Player p1, Player p2)
        {
            player1 = p1;
            player2 = p2;
            diceSums = GetDiceSums();
            visited = new Dictionary<(int, int, int, int),(long,long)> ();
        }

        private static IList<int> GetDiceSums()
        {
            var sums = new List<int>(27);
            for (var i = 1; i <= 3; i++)
            {
                for (var j = 1; j <= 3; j++)
                {
                    for (var z = 1; z <= 3; z++)
                    {
                        sums.Add(i + j + z);
                    }
                }
            }
            return sums;
        }

        internal (long,long) Start()
        {
            (long, long) totalWins = (0, 0);
            foreach (var dice in diceSums)
            {
                var (p1Wins, p2Wins) = Roll(dice,
                    player1.Position, player2.Position,
                    player1.Score, player2.Score);
                totalWins.Item1 += p1Wins;
                totalWins.Item2 += p2Wins;
            }

            return totalWins;
        }

        private (long,long) Roll(int move,
            int playerTurnPosition, int playerOtherPosition,
            int playerTurnScore, int playerOtherScore)
        {
            playerTurnPosition = (playerTurnPosition + move - 1) % 10 + 1;
            playerTurnScore += playerTurnPosition;
            if (playerTurnScore >= 21)
            {
                return (1, 0);
            }

            if(visited.TryGetValue((
                playerTurnPosition, playerOtherPosition,
                playerTurnScore, playerOtherScore), out var visitedWins)){
                return visitedWins;
            }

            (long,long) totalWins = (0, 0);
            foreach (var dice in diceSums)
            {
                var (pOtherWin, pCurrentWin) = Roll(dice,
                    playerOtherPosition, playerTurnPosition,
                    playerOtherScore, playerTurnScore);
                totalWins.Item1 += pCurrentWin;
                totalWins.Item2 += pOtherWin;
            }
            visited.Add((
                playerTurnPosition, playerOtherPosition,
                playerTurnScore, playerOtherScore), totalWins);

            return totalWins;
        }
    }
}