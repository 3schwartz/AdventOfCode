using Benchmark;
using BenchmarkDotNet.Configs;
using BenchmarkDotNet.Running;

BenchmarkRunner.Run<Day14Benchmark>(DefaultConfig.Instance, args);