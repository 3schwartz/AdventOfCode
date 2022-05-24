namespace Day22;

internal static class InstructionCreator
{
    internal static Instruction Create(string line)
    {
        var initial = line.Split(" ");
        Enum.TryParse(initial[0], true, out Turn turn);

        var coordinates = initial[1].Split(",");
        var x = GetFromTo(coordinates[0]);
        var y = GetFromTo(coordinates[1]);
        var z = GetFromTo(coordinates[2]);
        return new Instruction(
            turn,
            x.Item1, x.Item2,
            y.Item1, y.Item2,
            z.Item1, z.Item2);
    }

    private static (int, int) GetFromTo(string coordinate)
    {
        var initial = coordinate.Split("=");
        var coordinates = initial[1].Split("..");
        return (int.Parse(coordinates[0]), int.Parse(coordinates[1]));
    }
}