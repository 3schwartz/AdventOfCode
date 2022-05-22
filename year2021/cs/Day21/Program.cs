using Day21;

var p1 = new Player("Player 1 starting position: 8");
var p2 = new Player("Player 2 starting position: 5");
var game = new Game(p1, p2);

var result = game.Start();

Console.WriteLine($"Part 1: {result.GameResult}");