using System;
using System.Collections.Generic;
using System.Reactive.Disposables;
using System.Reactive.Subjects;
using System.Threading;
using System.Threading.Channels;
using System.Threading.Tasks;

namespace Day7.Tests;

internal class ReactiveCoder : ISubject<int>, IDisposable
{
    public string Identifier { get; }
    public Task? Work { get; private set; }
    private readonly IList<int> codes;
    private readonly Channel<int> channel;
    private readonly CancellationTokenSource cts;
    private int idx;
    private readonly List<int> outputs;

    private ReactiveCoder(string identifier, IList<int> codes)
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

    internal static async Task<IDictionary<string, ReactiveCoder>> SetupAmplifiers(IList<int> codes,
        (int A, int B, int C, int D, int E) inputs)
    {
        var a = await CreateReactiveCoder("A", new List<int>(codes), new List<int> { inputs.A, 0});
        var b = await CreateReactiveCoder("B", new List<int>(codes), new List<int> { inputs.B });
        var c = await CreateReactiveCoder("C", new List<int>(codes), new List<int> { inputs.C });
        var d = await CreateReactiveCoder("D", new List<int>(codes), new List<int> { inputs.D });
        var e = await CreateReactiveCoder("E", new List<int>(codes), new List<int> { inputs.E });
        a.Subscribe(b);
        b.Subscribe(c);
        c.Subscribe(d);
        d.Subscribe(e);
        e.Subscribe(a);
        return new Dictionary<string, ReactiveCoder>()
        {
            {a.Identifier, a},
            {b.Identifier, b},
            {c.Identifier, c},
            {d.Identifier, d},
            {e.Identifier, e},
        };
    }

    internal static async Task<ReactiveCoder> CreateReactiveCoder(string identifier, IList<int> codes, IList<int> initialInputs)
    {
        var coder = new ReactiveCoder(identifier, codes);
        foreach (var initialInput in initialInputs)
        {
            await coder.channel.Writer.WriteAsync( initialInput, coder.cts.Token);
        }
        return coder;
    }

    public void OnCompleted()
    {
        // Swallow
    }

    public void OnError(Exception error)
    {
        throw new NotImplementedException();
    }

    public void OnNext(int value)
    {
        channel.Writer.TryWrite(value);
    }

    public IDisposable Subscribe(IObserver<int> observer)
    {
        Work = Task.Run(() => Run(observer));
        return Disposable.Empty;
    }

    internal int GetLargestSignal()
    {
        return outputs[^1];
    }

    private async Task Run(IObserver<int> observer)
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
                        observer.OnNext(output);
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
                        observer.OnCompleted();
                        Dispose();
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