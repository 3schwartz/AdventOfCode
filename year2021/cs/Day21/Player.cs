namespace Day21;

internal class Player
{
    internal int Id { get; init; }
    internal int Position { get; private set; }
    internal int Score { get; private set; } = 0;

    public Player(string input)
    {
        var splitted = input.Split(" ");
        Id = int.Parse(splitted[1]);
        Position = int.Parse(splitted[4]);
    }

    public void Move(int move)
    {
        Position = (Position + move - 1) % 10 + 1;
        Score += Position;
    }
}