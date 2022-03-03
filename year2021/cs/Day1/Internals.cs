namespace Day1
{
    internal class Internals
    {
    }

    public static class FoldFactory
    {
        public static Fold CreateFold(string fold)
        {
            ReadOnlySpan<char> span = fold.AsSpan();

            FoldType foldType = GetFoldType(span[11]);

            bool parsedCoordinate = int.TryParse(span.Slice(13,13), out int coordinate);

            return new Fold(foldType, coordinate);
        }

        static FoldType GetFoldType(char f)
        {
            return f switch
            {
                'x' => FoldType.X,
                'y' => FoldType.Y,
                _ => throw new NotImplementedException(),
            };
        }

    }

    public record struct Fold(FoldType Along, int FoldCoordinate)
    {
    }

    public enum FoldType
    {
        X,
        Y
    }


}
