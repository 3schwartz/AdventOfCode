using System;
using System.Collections.Generic;
using System.Reactive.Disposables;
using System.Reactive.Subjects;
using System.Threading;
using System.Threading.Tasks;

namespace Day7.Tests;

internal class ReactiveCoder : ISubject<int>, IDisposable
{
    public string Identifier { get; }
    private readonly IList<int> codes;
    private readonly CancellationTokenSource cts;
    private readonly List<int> outputs;
    private readonly TaskCompletionSource<int> taskCompletionSource;
    private readonly Queue<int> inputs;
    private IObserver<int>? observer;
    private int idx;

    private ReactiveCoder(string identifier, IList<int> codes, int input)
    {
        Identifier = identifier;
        this.codes = codes;
        cts = new CancellationTokenSource();
        idx = 0;
        outputs = new List<int>();
        inputs = new Queue<int>();
        inputs.Enqueue(input);
        taskCompletionSource = new TaskCompletionSource<int>();
    }

    internal static int FindMaxThrusterSignal(IList<int> codes)
    {
        return CoderAlgorithms.FindMaxThrusterSignal(codes, FindThrusterSignal());
    }

    private static Func<IList<int>, (int A, int B, int C, int D, int E), Task<int>> FindThrusterSignal()
    {
        return async (codes, inputs) =>
        {
            var coders = SetupAmplifiers(codes, inputs);
            _ = Task.Run(() => coders["A"].OnNext(0));
            var coder = coders["E"];

            var thrusterSignal = await coder.GetLargestSignal()
                .ConfigureAwait(false);
            return thrusterSignal;
        };
    }

    internal static IDictionary<string, ReactiveCoder> SetupAmplifiers(IList<int> codes,
        (int A, int B, int C, int D, int E) inputs)
    {
        var a = new ReactiveCoder("A", new List<int>(codes), inputs.A);
        var b = new ReactiveCoder("B", new List<int>(codes), inputs.B);
        var c = new ReactiveCoder("C", new List<int>(codes), inputs.C);
        var d = new ReactiveCoder("D", new List<int>(codes), inputs.D);
        var e = new ReactiveCoder("E", new List<int>(codes), inputs.E);
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

    public void OnCompleted()
    {
        // swallow
    }

    public void OnError(Exception error)
    {
        taskCompletionSource.SetException(error);
    }

    public IDisposable Subscribe(IObserver<int> observer)
    {
        this.observer = observer;
        return Disposable.Empty;
    }

    private Task<int> GetLargestSignal()
    {
        return taskCompletionSource.Task;
    }

    public void OnNext(int input)
    {
        inputs.Enqueue(input);
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
                    if (!inputs.TryDequeue(out var outputQueue)) return;

                    codes[GetIdxFromMode(codes, execution, 1, idx)] = outputQueue;
                    idx += 2;
                    break;
                case 4:
                    var output = codes[GetIdxFromMode(codes, execution, 1, idx)];
                    outputs.Add(output);
                    idx += 2;
                    observer!.OnNext(output);
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
                    taskCompletionSource.TrySetResult(outputs[^1]);
                    observer!.OnCompleted();
                    Dispose();
                    return;
                default:
                    throw new Exception($"OptCode not known {execution}");
            }
        } while (true);
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