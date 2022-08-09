namespace Common
{
    /// <summary>
    /// Used for Day 11.
    /// </summary>
    internal partial class IntCoder
    {
        internal DefaultDict<(int, int), (int Color, int VisitedCount)> PaintHull(IList<long> codesInput)
        {
            (int X, int Y) currentPosition = (0, 0);
            (int X, int Y) direction = (0, 1);
            var outputCallCount = 0;
            // Don't need visisted count - refactor
            var visited = new DefaultDict<(int, int), (int Color, int VisitedCount)>();
            var codes = new DefaultDict<long, long>();
            for (int i = 0; i < codesInput.Count; i++)
            {
                codes.Add(i, codesInput[i]);
            }
            try
            {
                var inputCalled = 0;
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
                            codes[GetIdxFromMode(codes, execution, 1)] = visited[currentPosition].Color;
                            idx += 2;
                            inputCalled++;
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
                                visited[currentPosition] = (action, visited[currentPosition].VisitedCount + 1);
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

                return visited;
            }
            finally
            {
                idx = 0;
                relativeBase = 0;
            }
        }
    }
}
