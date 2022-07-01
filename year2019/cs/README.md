# Day 7
## Part 2
## Benchmark
``` ini

BenchmarkDotNet=v0.13.1, OS=Windows 10.0.19042.1766 (20H2/October2020Update)
Intel Core i7-8650U CPU 1.90GHz (Kaby Lake R), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.300
  [Host]     : .NET 6.0.5 (6.0.522.21309), X64 RyuJIT DEBUG  [AttachedDebugger]
  Job-VDURET : .NET 6.0.5 (6.0.522.21309), X64 RyuJIT

IterationCount=2  LaunchCount=2  WarmupCount=1  

```
|        Method |      Mean |     Error |    StdDev |    Gen 0 |  Gen 1 | Allocated |
|-------------- |----------:|----------:|----------:|---------:|-------:|----------:|
|      Channels | 15.149 ms | 3.3503 ms | 0.5185 ms | 812.5000 |      - |      3 MB |
| SortOfSubject |  1.863 ms | 0.5272 ms | 0.0816 ms | 394.5313 | 3.9063 |      2 MB |