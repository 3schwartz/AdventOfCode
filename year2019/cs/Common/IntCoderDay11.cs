namespace Common
{
    /// <summary>
    /// Used for Day 11.
    /// </summary>
    public partial class IntCoder
    {
        public void OutputHullPaint(
            DefaultDict<(int X, int Y), int> visited,
            Action<string> output)
        {
            (int xMin, int yMin, int xMax, int yMax) = (int.MaxValue, int.MaxValue, int.MinValue, int.MinValue);

            foreach (var (key, color) in visited)
            {
                if (key.X > xMax)
                {
                    xMax = key.X;
                }
                if (key.X < xMin)
                {
                    xMin = key.X;
                }
                if (key.Y > yMax)
                {
                    yMax = key.Y;
                }
                if (key.Y < yMin)
                {
                    yMin = key.Y;
                }
            }
            var xLength = Math.Abs(xMin) + Math.Abs(xMax) + 1;
            var yLength = Math.Abs(yMin) + Math.Abs(yMax) + 1;
            Span<char> rows = stackalloc char[yLength];
            for (var i = 0; i < xLength; i++)
            {
                for (var j = 0; j < yLength; j++)
                {
                    rows[j] = visited[(i - Math.Abs(xMin), j - Math.Abs(yMin))] == 1 ? '#' : '.';
                }
                output(rows.ToString());
            }
        }

        // Don't need visisted count - refactod
        public void PaintHullWithInput(IList<long> codesInput, DefaultDict<(int, int), int> visited)
        {
            (int X, int Y) currentPosition = (0, 0);
            (int X, int Y) direction = (0, 1);
            var outputCallCount = 0;
            var codes = new DefaultDict<long, long>();
            for (int i = 0; i < codesInput.Count; i++)
            {
                codes.Add(i, codesInput[i]);
            }
            try
            {
                do
                {
                    var execution = codes[idx];
                    switch (execution % 100)
                    {
                        case 1:
                            codes[GetIdxFromMode(codes, execution, 3)] =
                                codes[GetIdxFromMode(codes, execution, 2)] + codes[GetIdxFromMode(codes, execution, 1)];
                            idx += 4;
                            break;
                        case 2:
                            codes[GetIdxFromMode(codes, execution, 3)] =
                                codes[GetIdxFromMode(codes, execution, 2)] * codes[GetIdxFromMode(codes, execution, 1)];
                            idx += 4;
                            break;
                        case 3:
                            codes[GetIdxFromMode(codes, execution, 1)] = visited[currentPosition];
                            idx += 2;
                            break;
                        case 4:
                            outputCallCount++;
                            var output = codes[GetIdxFromMode(codes, execution, 1)];
                            idx += 2;
                            var action = output switch
                            {
                                0 => 0,
                                1 => 1,
                                _ => throw new Exception($"Output not correct: {output}")
                            };
                            if (outputCallCount % 2 == 1)
                            {
                                visited[currentPosition] = action;
                                continue;
                            }
                            switch (action)
                            {
                                case 0:
                                    direction = (-direction.Y, direction.X);
                                    break;
                                case 1:
                                    direction = (direction.Y, -direction.X);
                                    break;
                            }
                            currentPosition = (currentPosition.X + direction.X, currentPosition.Y + direction.Y);
                            break;
                        case 5:
                            if (codes[GetIdxFromMode(codes, execution, 1)] != 0)
                            {
                                idx = codes[GetIdxFromMode(codes, execution, 2)];
                                break;
                            }
                            idx += 3;
                            break;
                        case 6:
                            if (codes[GetIdxFromMode(codes, execution, 1)] == 0)
                            {
                                idx = codes[GetIdxFromMode(codes, execution, 2)];
                                break;
                            }
                            idx += 3;
                            break;
                        case 7:
                            codes[GetIdxFromMode(codes, execution, 3)] =
                                codes[GetIdxFromMode(codes, execution, 1)] < codes[GetIdxFromMode(codes, execution, 2)] ?
                                1 : 0;
                            idx += 4;
                            break;
                        case 8:
                            codes[GetIdxFromMode(codes, execution, 3)] =
                                codes[GetIdxFromMode(codes, execution, 1)] == codes[GetIdxFromMode(codes, execution, 2)] ?
                                1 : 0;
                            idx += 4;
                            break;
                        case 9:
                            relativeBase += codes[GetIdxFromMode(codes, execution, 1)];
                            idx += 2;
                            break;
                        case 99:
                            idx = codes.Count;
                            break;
                        default:
                            throw new Exception($"OptCode not known {execution}");
                    }
                } while (idx < codes.Count);
            }
            finally
            {
                idx = 0;
                relativeBase = 0;
            }
        }

        public DefaultDict<(int, int), int> PaintHull(IList<long> codesInput)
        {
            var visited = new DefaultDict<(int, int), int>();
            PaintHullWithInput(codesInput, visited);
            return visited;
        }
    }
}
