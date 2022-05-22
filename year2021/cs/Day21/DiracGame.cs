
namespace Day21
{
    internal class DiracGame
    {
        private readonly HashSet<(int, int, int, int)> visited;
        private readonly IList<int> diceSums;
        private Player player1;
        private Player player2;

        public DiracGame(Player p1, Player p2)
        {
            player1 = p1;
            player2 = p2;
            diceSums = GetDiceSums();
            visited = new HashSet<(int, int, int, int)> ();
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

        internal (int,int) Start()
        {
            (int, int) totalWins = (0, 0);
            foreach (var dice in diceSums)
            {
                var wins = Roll(1, dice,
                    player1.Position, player2.Position,
                    player1.Score, player2.Score);
                totalWins.Item1 += wins.Item1;
                totalWins.Item2 += wins.Item2;
            }

            return totalWins;
        }

        private (int,int) Roll(int currentPlayer, int move,
            int playerTurnPosition, int playerOtherPosition,
            int playerTurnScore, int playerOtherScore)
        {
            playerTurnPosition = (playerTurnPosition + move - 1) % 10 + 1;
            playerTurnScore += playerTurnPosition;
            if (playerTurnScore >= 21)
            {
                return currentPlayer == 1 ? (1, 0) : (0, 1);
            }

            (int, int, int, int) visitedWins;
            var isVisited = currentPlayer switch
            {
                1 => visited.TryGetValue(
                    (playerTurnPosition, playerOtherPosition,
                        playerTurnScore, playerOtherScore), out visitedWins),
                2 => visited.TryGetValue(
                    (playerOtherPosition, playerTurnPosition,
                        playerOtherScore, playerTurnScore), out visitedWins)
            };
            if (isVisited)
            {
                return (visitedWins.Item3, visitedWins.Item4);
            }

            switch (currentPlayer)
            {
                case 1:
                    visited.Add((playerTurnPosition, playerOtherPosition,
                        playerTurnScore, playerOtherScore));
                    break;
                case 2:
                    visited.Add((playerOtherPosition, playerTurnPosition,
                        playerOtherScore, playerTurnScore));
                    break;
            }

            if (visited.TryGetValue(
                (playerTurnPosition, playerOtherPosition,
                playerTurnScore, playerOtherScore), out var winsVisited))
            {
                return (winsVisited.Item3, winsVisited.Item4);
            }

            var totalWins = (0, 0);
            foreach (var dice in diceSums)
            {
                var wins = Roll(currentPlayer % 2 + 1, dice,
                    playerOtherPosition, playerTurnPosition,
                    playerOtherScore, playerTurnScore);
                totalWins.Item1 += wins.Item1;
                totalWins.Item2 += wins.Item2;
            }

            switch (currentPlayer)
            {
                case 1:
                    visited.Add((playerTurnPosition, playerOtherPosition,
                        playerTurnScore, playerOtherScore));
                    break;
                case 2:
                    visited.Add((playerOtherPosition, playerTurnPosition,
                        playerOtherScore, playerTurnScore));
                    break;
            }

            return totalWins;
        }
    }
}