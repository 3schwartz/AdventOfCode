# 2019 Benchmarks

## Day 4

#### Go

````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Benchmark_mapIntegersFromAsciiBytes/Pointers-8 | 693726696 | 1.815 ns/op |  0 B/op | 0 allocs/op |
| Benchmark_mapIntegersFromAsciiBytes/Converter-8 | 1000000000 |  0.8019 ns/op | 0 B/op	| 0 allocs/op |

## Day 7
### Part 2

#### C#
``` ini

BenchmarkDotNet=v0.13.1
Intel Core i7-8650U CPU 1.90GHz (Kaby Lake R), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.300

IterationCount=2  LaunchCount=2  WarmupCount=1  

```
|        Method |      Mean |     Error |    StdDev |    Gen 0 |  Gen 1 | Allocated |
|-------------- |----------:|----------:|----------:|---------:|-------:|----------:|
|      Channels | 15.149 ms | 3.3503 ms | 0.5185 ms | 812.5000 |      - |      3 MB |
| SortOfSubject |  1.863 ms | 0.5272 ms | 0.0816 ms | 394.5313 | 3.9063 |      2 MB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| ChannelCoder | 187 | 6.390699 ms/op |  3_020_306 B/op | 5_520 allocs/op |
| ReactiveCoder | 2_378 |  0.933929 ms/op | 2_549_760 B/op	| 3_720 allocs/op |

## Day 10
### Part 1

#### C#
``` ini

BenchmarkDotNet=v0.13.1, OS=Windows 10.0.19043.1826 (21H1/May2021Update)
Intel Core i7-8565U CPU 1.80GHz (Whiskey Lake), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.302
  [Host]     : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT
  DefaultJob : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT


```
|                 Method |      Mean |     Error |    StdDev |     Gen 0 |   Gen 1 | Allocated |
|----------------------- |----------:|----------:|----------:|----------:|--------:|----------:|
| DetectedAsteroidsAsync |  4.469 ms | 0.0759 ms | 0.0710 ms | 2335.9375 | 15.6250 |      8 MB |
|  DetectedAsteroidsSync | 10.812 ms | 0.2835 ms | 0.7948 ms | 1937.5000 |       - |      8 MB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Trigonometric-8 | 54 | 23_788_604 ns/op (23.78 ms/op) |  11_312_955 B/op ~ 11 MB | 10_112 allocs/op |
| Slopes-8 | 24 |  43_277_388 ns/op (43.27 ms/op) | 26_828_667 B/op ~ 26 MB	| 21_284 allocs/op |

## Day 12
### Part 2

#### C#

Need to subtract SetupWithinEachBenchmark from the others. Can't use `[IterationSetup]` [since test below 100ms](https://benchmarkdotnet.org/articles/samples/IntroSetupCleanupIteration.html).

``` ini

BenchmarkDotNet=v0.13.1, OS=Windows 10.0.19043.1826 (21H1/May2021Update)
Intel Core i7-8565U CPU 1.80GHz (Whiskey Lake), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.302
  [Host]     : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT
  DefaultJob : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT


```
|                       Method |         Mean |       Error |     StdDev |       Median |    Ratio | RatioSD |     Gen 0 | Allocated |
|----------------------------- |-------------:|------------:|-----------:|-------------:|---------:|--------:|----------:|----------:|
|     SetupWithinEachBenchmark |     7.643 μs |   0.9430 μs |   2.780 μs |     6.323 μs |     1.00 |    0.00 |    1.6327 |      7 KB |
|      StepsToGetBackToInitial | 9,155.421 μs (9.155 ms) | 284.7106 μs | 774.576 μs | 8,911.399 μs | 1,349.78 |  386.71 | 1875.0000 |  7,704 KB |
| StepsToGetBackToInitialAsync | 6,093.327 μs (6.093 ms) | 120.9209 μs | 107.193 μs | 6,074.534 μs |   871.71 |  161.86 | 1890.6250 |  7,705 KB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Setup-8 | 484393 | 2_339 ns/op (2.339 μs / op)) |  896 B/op | 23 allocs/op |
| Sync-8 | 3 |  450_950_267 ns/op (460.950 ms/op) | 149_761_752 B/op ~ 149 MB	| 6_760_006 allocs/op |
| Async-8 | 4 |  299_714_200 ns/op (299.714 ms/op) | 149_763_040 B/op ~ 149 MB	| 6_760_021 allocs/op |