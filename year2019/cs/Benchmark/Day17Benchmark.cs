using BenchmarkDotNet.Attributes;
using Common;
using Day7.Tests;

namespace Benchmark;

[MemoryDiagnoser]
[SimpleJob(launchCount: 2, warmupCount: 1, targetCount: 2)]
public class Day17Benchmark
{
    private IList<int>? codes;

    [GlobalSetup]
    public async Task Setup()
    {
        codes = IntCoder.InputToCodes(await File.ReadAllTextAsync("../../../../../../../../../data/day7_data.txt"));
    }

    [Benchmark]
    public void Channels()
    {
        _ = ChannelCoder.FindMaxThrusterSignal(codes!);
    }

    [Benchmark]
    public void SortOfSubject()
    {
        _ = ReactiveCoder.FindMaxThrusterSignal(codes!);
    }
}