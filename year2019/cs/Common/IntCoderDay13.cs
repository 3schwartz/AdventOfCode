namespace Common
{
    public class GameState
    {
        private int outputCount;
        private long ballX;
        private long paddleX;
        private long x;
        private long y;
        public long TotalScore { get; private set; }
        public long BlockCount { get; private set; }

        internal int GetInput()
        {
            if (ballX < paddleX) return -1;
            if (ballX > paddleX) return 1;
            return 0;
        }

        internal void UpdateFromOutput(long output)
        {
            switch (outputCount % 3)
            {
                case 0:
                    x = output;
                    break;
                case 1:
                    y = output;
                    break;
                case 2:
                    if (x == -1 && y == 0)
                    {
                        TotalScore = output;
                        break;
                    }
                    switch (output) {
                        case 2:
                            BlockCount++;
                            break;
                        case 3:
                            paddleX = x;
                            break;
                        case 4:
                            ballX = x;
                            break;
                    }
                    break;
            }
            outputCount++;
        } 
    }

    public partial class IntCoder
    {
        public GameState PlayArcade(IList<int> codesInput)
        {
            var codes = new DefaultDict<long, long>();
            for (int i = 0; i < codesInput.Count; i++)
            {
                codes.Add(i, codesInput[i]);
            }
            var state = new GameState();
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
                            codes[GetIdxFromMode(codes, execution, 1)] = state.GetInput();
                            idx += 2;
                            break;
                        case 4:
                            var output = codes[GetIdxFromMode(codes, execution, 1)];
                            state.UpdateFromOutput(output);
                            idx += 2;
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

                return state;
            }
            finally
            {
                idx = 0;
                relativeBase = 0;
            }
        }
    }
}