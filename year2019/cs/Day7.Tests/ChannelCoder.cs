using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Channels;
using System.Threading.Tasks;

namespace Day7.Tests;

internal class ChannelCoder : IDisposable
{
    public string Identifier { get; }
    public Task? Work { get; private set; }
    private readonly IList<int> codes;
    private readonly Channel<int> channel;
    private readonly CancellationTokenSource cts;
    private int idx;
    private ChannelCoder? observer;
    private readonly List<int> outputs;

    private ChannelCoder(string identifier, IList<int> codes)
    {
        Identifier = identifier;
        this.codes = codes;
        channel = Channel.CreateUnbounded<int>();
        cts = new CancellationTokenSource();
        idx = 0;
        outputs = new List<int>();
    }

    internal static int FindMaxThrusterSignal(IList<int> codes)
    {
        return CoderAlgorithms.FindMaxThrusterSignal(codes, FindThrusterSignal());
    }

    private static Func<IList<int>, (int A, int B, int C, int D, int E), Task<int>> FindThrusterSignal()
    {
        return async (codes, inputs) =>
        {
            var coders = await SetupAmplifiers(codes, inputs)
                .ConfigureAwait(false);
            var coder = coders["E"];

            await coder.Work!
                .ConfigureAwait(false);
            var thrusterSignal = coder.GetLargestSignal();
            return thrusterSignal;
        };
    }

    internal static async Task<IDictionary<string, ChannelCoder>> SetupAmplifiers(IList<int> codes,
        (int A, int B, int C, int D, int E) inputs)
    {
        var a = await CreateChannelCoder("A", new List<int>(codes), new List<int> { inputs.A, 0});
        var b = await CreateChannelCoder("B", new List<int>(codes), new List<int> { inputs.B });
        var c = await CreateChannelCoder("C", new List<int>(codes), new List<int> { inputs.C });
        var d = await CreateChannelCoder("D", new List<int>(codes), new List<int> { inputs.D });
        var e = await CreateChannelCoder("E", new List<int>(codes), new List<int> { inputs.E });
        a.Attach(b);
        b.Attach(c);
        c.Attach(d);
        d.Attach(e);
        e.Attach(a);
        return new Dictionary<string, ChannelCoder>()
        {
            {a.Identifier, a},
            {b.Identifier, b},
            {c.Identifier, c},
            {d.Identifier, d},
            {e.Identifier, e},
        };
    }

    internal static async Task<ChannelCoder> CreateChannelCoder(string identifier, IList<int> codes, IList<int> initialInputs)
    {
        var coder = new ChannelCoder(identifier, codes);
        foreach (var initialInput in initialInputs)
        {
            await coder.channel.Writer.WriteAsync( initialInput, coder.cts.Token);
        }
        return coder;
    }

    internal int GetLargestSignal()
    {
        return outputs[^1];
    }


    internal async Task Notify(int input, CancellationToken ct)
    {
        await channel.Writer.WriteAsync(input, ct);
    }

    internal void Attach(ChannelCoder coder)
    {
        observer = coder;
        Work = Task.Run(Run);
    }

    private async Task Run()
    {
        await foreach (var input in channel.Reader.ReadAllAsync(cts.Token))
        {
            var inputUsed = false;
            var keepWhileLoopOn = true;
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
                        if (inputUsed)
                        {
                            keepWhileLoopOn = false;
                            break;
                        }
                        codes[GetIdxFromMode(codes, execution, 1, idx)] = input;
                        idx += 2;
                        inputUsed = true;
                        break;
                    case 4:
                        var output = codes[GetIdxFromMode(codes, execution, 1, idx)];
                        await (observer != null ? observer.Notify(output, cts.Token) : null);
                        outputs.Add(output);
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
                        return;
                    default:
                        throw new Exception($"OptCode not known {execution}");
                }
            } while (keepWhileLoopOn);
        }
    }

    private static int GetIdxFromMode(IList<int> codes, int execution, int parameterPosition, int idx)
    {
        var mode = execution / (int)Math.Pow(10, 1 + parameterPosition);
        mode %= 10;
        return mode == 1 ? idx + parameterPosition : codes[idx + parameterPosition];

    }

    public void Dispose()
    {
        cts.Dispose();
    }
}