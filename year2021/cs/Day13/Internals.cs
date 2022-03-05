namespace Day1
{
    public class Grid
    {
        private int[][] grid;

        private Grid() { }

        public Grid(int xSize, int ySize)
        {
            grid = CreateGrid(xSize, ySize);
        }

        private static int[][] CreateGrid(int xSize, int ySize)
        {
            var grid = new int[ySize][];

            for (var i = 0; i < grid.Length; i++)
            {
                grid[i] = new int[xSize];
                for (var j = 0; j < grid[i].Length; j++)
                {
                    grid[i][j] = 0;
                }
            }

            return grid;
        }

        public int[][] GetGrid()
        {
            return grid;
        }

        public void PopulateGrid(Coordinate[] coords)
        {
            foreach (var (x, y) in coords)
            {
                grid[y][x] = 1;
            }
        }
        public void Fold(Fold fold)
        {
            grid = fold.Along switch
            {
                FoldType.X => FoldAlongX(fold),
                FoldType.Y => FoldAlongY(fold),
                _ => throw new NotSupportedException($"Fold type not supported {fold.Along}")
            };
        }

        public int GetNumberOfDots()
        {
            var sum = 0;
            for (int i = 0; i < grid.Length; i++)
            {
                for (int j = 0; j < grid[i].Length; j++)
                {
                    sum += grid[i][j];
                }
            }

            return sum;
        }

        public void Print()
        {
            foreach (var i in grid)
            {
                foreach (var i1 in i)
                {
                    Console.Write(i1 == 1 ? "#" : ".");
                }
                Console.WriteLine();
            }
        }

        private int[][] FoldAlongY(Fold fold)
        {
            var newGrid = InitializeNewGridFromOld(fold.FoldCoordinate, grid[0].Length);

            for (var i = grid.Length - 1; fold.FoldCoordinate < i; i--)
            {
                for (var j = 0; j < grid[i].Length; j++)
                {
                    if (grid[i][j] == 1)
                    {
                        newGrid[grid.Length - 1 - i][j] = grid[i][j];
                    }
                }
            }

            return newGrid;
        }

        private int[][] FoldAlongX(Fold fold)
        {
            var newGrid = InitializeNewGridFromOld(grid.Length, fold.FoldCoordinate);

            for (var i = 0; i < grid.Length; i++)
            {
                for (var j = grid[0].Length - 1; fold.FoldCoordinate < j; j--)
                {
                    if (grid[i][j] == 1)
                    {
                        newGrid[i][grid[0].Length - 1 - j] = grid[i][j];
                    }
                }
            }

            return newGrid;
        }

        private int[][] InitializeNewGridFromOld(int newXSize, int newYSize)
        {
            var newGrid = new int[newXSize][];

            for (var i = 0; i < newGrid.Length; i++)
            {
                newGrid[i] = new int[newYSize];
                for (var j = 0; j < newGrid[i].Length; j++)
                {
                    newGrid[i][j] = grid[i][j];
                }
            }

            return newGrid;
        }
    }

    public record struct Coordinate(int X, int Y);

    public static class CoordinateFactory
    {
        public static Coordinate[] CreateCoordinates(string coordinates)
        {
            var strings = coordinates.Split("\r\n");
            var coords = new Coordinate[strings.Length];

            for (var i = 0; i < strings.Length; i++)
            {
                var spittedCoords = strings[i].Split(",");
                coords[i] = new Coordinate(int.Parse(spittedCoords[0]), int.Parse(spittedCoords[1]));
            }

            return coords;
        }
    }

    public static class FoldFactory
    {
        public static Fold CreateFold(string fold)
        {
            ReadOnlySpan<char> span = fold.AsSpan();

            FoldType foldType = GetFoldType(span[11]);

            bool parsedCoordinate = int.TryParse(span.Slice(13,span.Length - 13),
                out int coordinate);

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

    public record struct Fold(FoldType Along, int FoldCoordinate);

    public enum FoldType
    {
        X,
        Y
    }


}
