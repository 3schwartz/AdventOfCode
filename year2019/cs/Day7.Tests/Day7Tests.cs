using System;
using System.Collections.Generic;
using Common;
using System.IO;
using System.Threading;
using System.Threading.Channels;
using System.Threading.Tasks;
using Xunit;
using Xunit.Abstractions;

namespace Day7.Tests;

public class Day7Tests
{
    private readonly ITestOutputHelper output;

    public Day7Tests(ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Program()
    {
        var codes = IntCoder.InputToCodes(File.ReadAllText("../../../../../data/day7_data.txt"));
        var coder = new IntCoder();

        var maxThrusterSignal = coder.FindMaxThrusterSignal(codes);

        output.WriteLine($"Part 1: {maxThrusterSignal}");

        var maxLoopThrusterSignal = ChannelCoder.FindMaxThrusterSignal(codes);

        output.WriteLine($"Part 2: {maxLoopThrusterSignal}");
    }

    [Theory]
    [InlineData("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", 139629729)]
    [InlineData("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", 18216)]
    public void GivenChannelCoder_WhenFindLoopThruster_ThenCorrect(string input, int thrusterSignal)
    {
        // Arrange
        var codes = IntCoder.InputToCodes(input);

        // Act
        var maxThrusterSignal = ChannelCoder.FindMaxThrusterSignal(codes);

        // Assert
        Assert.Equal(thrusterSignal, maxThrusterSignal);
    }

    [Theory]
    [InlineData("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210)]
    [InlineData("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", 54321)]
    [InlineData("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210)]
    public void WhenFindHighestShoot_ThenCorrect(string input, int thrusterSignal)
    {
        // Arrange
        var codes = IntCoder.InputToCodes(input);
        var coder = new IntCoder();

        // Act
        var maxThrusterSignal = coder.FindMaxThrusterSignal(codes);

        // Assert
        Assert.Equal(thrusterSignal, maxThrusterSignal);
    }
}

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
        return IntCoder.FindMaxThrusterSignal(codes, FindThrusterSignal());
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