namespace Common;

internal class IntCoder
{

    private class DefaultDict<TKey, TValue> : Dictionary<TKey, TValue> where TValue : new() where TKey : notnull
    {
        public new TValue this[TKey key]
        {
            get
            {
                if (!TryGetValue(key, out var val))
                {
                    val = new TValue();
                    Add(key, val);
                }
                return val;
            }
            set { base[key] = value; }
        }

    }

    internal static IList<int> InputToCodes(string input)
    {
        return input
        .Split(",")
        .Select(c => int.Parse(c))
        .ToList();
    }

    private long idx = 0;
    private long relativeBase = 0;


    internal long RunTest(IList<int> codesInput, Func<long, long> getInput)
    {
        var codes = new DefaultDict<long, long>();
        for (int i = 0; i < codesInput.Count; i++)
        {
            codes.Add(i, codesInput[i]);
        }
        try
        {
            var inputCalled = 0;
            var outputs = new List<long>();
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
                        codes[GetIdxFromMode(codes, execution, 1)] = getInput(inputCalled);
                        idx += 2;
                        inputCalled++;
                        break;
                    case 4:
                        outputs.Add(codes[GetIdxFromMode(codes, execution, 1)]);
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

            return outputs[^1];
        }
        finally
        {
            idx = 0;
            relativeBase = 0;
        }
    }
    internal long FindMaxThrusterSignal(IList<int> codes)
    {
        Span<long> inputs = stackalloc long[5] { -1,-1,-1,-1,-1};
        long last;
        long maxSignal = int.MinValue;

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

    private long GetIdxFromMode(IDictionary<long, long> codes, long execution, int parameterPosition)
    {
        var mode = execution / (int)Math.Pow(10, 1 + parameterPosition);
        mode %= 10;
        return mode switch
        {
            0 => codes[idx + parameterPosition],
            1 => idx + parameterPosition,
            2 => codes[idx + parameterPosition] + relativeBase,
            _ => throw new ArgumentOutOfRangeException(nameof(mode), "Mode not known"),
        };
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