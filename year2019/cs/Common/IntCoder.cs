namespace Common;

internal class IntCoder
{

    internal static IList<int> InputToCodes(string input)
    {
        return input
        .Split(",")
        .Select(c => int.Parse(c))
        .ToList();
    }


    internal int RunTest(IList<int> codes, Func<int,int> getInput)
    {
        var idx = 0;
        var inputCalled = 0;
        var outputs = new List<int>();
        do
        {
            var execution = codes[idx];
            switch (execution % 100)
            {
                case 1:
                    codes[GetIdxFromMode(codes, execution, 3, idx)] =
                        codes[GetIdxFromMode(codes, execution, 2, idx)] + codes[GetIdxFromMode(codes, execution, 1, idx)];
                    idx += 4;
                    break;
                case 2:
                    codes[GetIdxFromMode(codes, execution, 3, idx)] =
                        codes[GetIdxFromMode(codes, execution, 2, idx)] * codes[GetIdxFromMode(codes, execution, 1, idx)];
                    idx += 4;
                    break;
                case 3:
                    codes[GetIdxFromMode(codes, execution, 1, idx)] = getInput(inputCalled);
                    idx += 2;
                    inputCalled++;
                    break;
                case 4:
                    outputs.Add(codes[GetIdxFromMode(codes, execution, 1, idx)]);
                    idx += 2;
                    break;
                case 5:
                    if (codes[GetIdxFromMode(codes, execution, 1, idx)] != 0)
                    {
                        idx = codes[GetIdxFromMode(codes, execution, 2, idx)];
                        break;
                    }
                    idx += 3;
                    break;
                case 6:
                    if (codes[GetIdxFromMode(codes, execution, 1, idx)] == 0)
                    {
                        idx = codes[GetIdxFromMode(codes, execution, 2, idx)];
                        break;
                    }
                    idx += 3;
                    break;
                case 7:
                    codes[GetIdxFromMode(codes, execution, 3, idx)] =
                        codes[GetIdxFromMode(codes, execution, 1, idx)] < codes[GetIdxFromMode(codes, execution, 2, idx)] ?
                        1 : 0;
                    idx += 4;
                    break;
                case 8:
                    codes[GetIdxFromMode(codes, execution, 3, idx)] =
                        codes[GetIdxFromMode(codes, execution, 1, idx)] == codes[GetIdxFromMode(codes, execution, 2, idx)] ?
                        1 : 0;
                    idx += 4;
                    break;
                case 99:
                    idx = codes.Count;
                    break;
                default:
                    throw new Exception($"OptCode not known {execution}");
            }
        } while (idx < codes.Count);

        return outputs[^1];
    }

    internal static int FindMaxThrusterSignal(IList<int> codes, 
        Func<IList<int>, (int A, int B, int C, int D, int E), Task<int>> findThrusterSignal)
    {
        Span<int> inputs = stackalloc int[5] { -1, -1, -1, -1, -1 };
        int last;
        int maxSignal = int.MinValue;

        for (int first = 5; first < 10; first++)
        {
            inputs[0] = first;
            for (int second = 5; second < 10; second++)
            {
                if (inputs.Contains(second)) continue;
                inputs[1] = second;

                for (int third = 5; third < 10; third++)
                {
                    if (inputs.Contains(third)) continue;
                    inputs[2] = third;

                    for (int fourth = 5; fourth < 10; fourth++)
                    {
                        if (inputs.Contains(fourth)) continue;
                        inputs[3] = fourth;

                        for (int fifth = 5; fifth < 10; fifth++)
                        {
                            if (inputs.Contains(fifth)) continue;
                            inputs[4] = fifth;

                            last = findThrusterSignal(codes,
                                (inputs[0], inputs[1], inputs[2], inputs[3], inputs[4]))
                                .Result;

                            if (last > maxSignal)
                            {
                                maxSignal = last;
                            }
                            inputs[4] = -1;
                        }
                        inputs[3] = -1;
                    }
                    inputs[2] = -1;
                }
                inputs[1] = -1;
            }
            inputs[0] = -1;
        }
        return maxSignal;
    }

    internal int FindMaxThrusterSignal(IList<int> codes)
    {
        Span<int> inputs = stackalloc int[5] { -1,-1,-1,-1,-1};
        int last;
        int maxSignal = int.MinValue;

        for (int first = 0; first < 5; first++)
        {
            inputs[0] = first;
            for (int second = 0; second < 5; second++)
            {
                if (inputs.Contains(second)) continue;
                inputs[1] = second;

                for (int third = 0; third < 5; third++)
                {
                    if (inputs.Contains(third)) continue;
                    inputs[2] = third;

                    for (int fourth = 0; fourth < 5; fourth++)
                    {
                        if (inputs.Contains(fourth)) continue;
                        inputs[3] = fourth;

                        for (int fifth = 0; fifth < 5; fifth++)
                        {
                            if (inputs.Contains(fifth)) continue;
                            inputs[4] = fifth;

                            last = 0;
                            for (int i = 0; i < 5; i++)
                            {
                                var input = inputs[i];
                                last = RunTest(new List<int>(codes), (idx) => idx == 0 ? input : last);
                            }
                            if(last > maxSignal)
                            {
                                maxSignal = last;
                            }
                            inputs[4] = -1;
                        }
                        inputs[3] = -1;
                    }
                    inputs[2] = -1;
                }
                inputs[1] = -1;
            }
            inputs[0] = -1;
        }
        return maxSignal;
    }

    private int GetIdxFromMode(IList<int> codes, int execution, int parameterPosition, int idx)
    {
        var mode = execution / (int)Math.Pow(10, 1 + parameterPosition);
        mode %= 10;
        return mode == 1 ? idx + parameterPosition : codes[idx + parameterPosition];

    }

    internal void RunInstructions(IList<int> codes, int noun, int verb)
    {
        codes[1] = noun;
        codes[2] = verb;
        var idx = 0;
        do
        {
            var optcode = codes[idx];
            switch (optcode)
            {
                case 1:
                    codes[codes[idx + 3]] = codes[codes[idx + 2]] + codes[codes[idx + 1]];
                    break;
                case 2:
                    codes[codes[idx + 3]] = codes[codes[idx + 2]] * codes[codes[idx + 1]];
                    break;
                case 99:
                    idx = codes.Count;
                    break;
                default:
                    throw new Exception($"OptCode not known {optcode}");
            }
            idx += 4;
        } while (idx < codes.Count);
    }

    internal int FindOutput(IList<int> codes)
    {
        for (int i = 0; i <= 99; i++)
        {
            for (int j = 0; j <= 99; j++)
            {
                var copy = new List<int>(codes);
                RunInstructions(copy, i, j);
                if (copy[0] == 19690720)
                {
                    return 100 * i + j;
                }
            }
        }
        throw new Exception("Nothing found");
    }
}