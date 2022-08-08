using Benchmark;
using BenchmarkDotNet.Configs;
using BenchmarkDotNet.Running;

//BenchmarkRunner.Run<Day7Benchmark>(DefaultConfig.Instance.WithOptions(ConfigOptions.DisableOptimizationsValidator));
BenchmarkRunner.Run<Day10Benchmark>(DefaultConfig.Instance.WithOptions(ConfigOptions.DisableOptimizationsValidator));